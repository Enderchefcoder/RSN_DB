use petgraph::graph::UnGraph;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChunk {
    pub id: String,
    pub text: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: String,
    pub mentions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    pub id: usize,
    pub entities: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphRagData {
    pub chunks: HashMap<String, TextChunk>,
    pub entities: HashMap<String, Entity>,
    pub relations: Vec<Relation>,
    pub communities: Vec<Community>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphRagEngine {
    pub data: GraphRagData,
    #[serde(skip)]
    tfidf_index: HashMap<String, HashMap<String, f32>>, // word -> {chunk_id -> score}
}

impl GraphRagEngine {
    pub fn new() -> Self {
        Self {
            data: GraphRagData::default(),
            tfidf_index: HashMap::new(),
        }
    }

    pub fn ingest(&mut self, text: &str, source: &str) {
        let chunks = self.chunk_text(text, source);
        for chunk in chunks {
            let extracted_entities = self.extract_entities(&chunk.text);
            let extracted_relations = self.extract_relations(&chunk.text, &extracted_entities);

            for ent in extracted_entities {
                self.data.entities.entry(ent.name.clone())
                    .and_modify(|e| e.mentions += 1)
                    .or_insert(ent);
            }

            for rel in extracted_relations {
                self.data.relations.push(rel);
            }

            self.data.chunks.insert(chunk.id.clone(), chunk);
        }
        self.rebuild_tfidf();
        self.detect_communities();
    }

    fn chunk_text(&self, text: &str, source: &str) -> Vec<TextChunk> {
        let mut chunks = Vec::new();
        let sentences: Vec<&str> = text.split_inclusive(&['.', '!', '?'][..]).collect();

        let chunk_size = 500;
        let mut current_chunk = String::new();
        let mut count = 0;

        for sent in sentences {
            if current_chunk.len() + sent.len() > chunk_size && !current_chunk.is_empty() {
                let id = format!("{:x}", Sha256::digest(format!("{}_{}", source, count).as_bytes()))[..12].to_string();
                chunks.push(TextChunk {
                    id,
                    text: current_chunk.clone(),
                    source: source.to_string(),
                });
                current_chunk.clear();
                count += 1;
            }
            current_chunk.push_str(sent);
        }

        if !current_chunk.is_empty() {
            let id = format!(
                "{:x}",
                Sha256::digest(format!("{}_{}", source, count).as_bytes())
            )[..12]
                .to_string();
            chunks.push(TextChunk {
                id,
                text: current_chunk,
                source: source.to_string(),
            });
        }
        chunks
    }

    fn extract_entities(&self, text: &str) -> Vec<Entity> {
        let mut entities = HashMap::new();
        let Ok(re) = Regex::new(r"\b[A-Z][a-z]+(?:\s+[A-Z][a-z]+)*\b") else {
            return Vec::new();
        };
        for mat in re.find_iter(text) {
            let name = mat.as_str().to_string();
            if name.len() > 2 {
                entities.entry(name.clone()).or_insert(Entity {
                    name,
                    entity_type: "CONCEPT".to_string(),
                    mentions: 1,
                });
            }
        }
        entities.into_values().collect()
    }

    fn extract_relations(&self, text: &str, entities: &[Entity]) -> Vec<Relation> {
        let mut relations = Vec::new();
        for i in 0..entities.len() {
            for j in i + 1..entities.len() {
                let e1 = &entities[i];
                let e2 = &entities[j];
                if text.contains(&e1.name) && text.contains(&e2.name) {
                    relations.push(Relation {
                        source: e1.name.clone(),
                        target: e2.name.clone(),
                        relation_type: "CO_OCCURS".to_string(),
                        weight: 1.0,
                    });
                }
            }
        }
        relations
    }

    pub fn rebuild_tfidf(&mut self) {
        let mut doc_counts: HashMap<String, usize> = HashMap::new();
        let num_docs = self.data.chunks.len();

        for chunk in self.data.chunks.values() {
            let lower = chunk.text.to_lowercase();
            let words: HashSet<_> = lower.split_whitespace().collect();
            for word in words {
                *doc_counts.entry(word.to_string()).or_insert(0) += 1;
            }
        }

        self.tfidf_index.clear();
        for (cid, chunk) in &self.data.chunks {
            let lower = chunk.text.to_lowercase();
            let words: Vec<_> = lower.split_whitespace().collect();
            let mut word_counts = HashMap::new();
            for word in &words {
                *word_counts.entry(word).or_insert(0) += 1;
            }

            for (word, count) in word_counts {
                let tf = count as f32 / words.len() as f32;
                let idf = ((num_docs as f32) / (*doc_counts.get(*word).unwrap_or(&1) as f32)).ln();
                self.tfidf_index
                    .entry(word.to_string())
                    .or_insert_with(HashMap::new)
                    .insert(cid.clone(), tf * idf);
            }
        }
    }

    pub fn detect_communities(&mut self) {
        let mut graph = UnGraph::<String, f32>::new_undirected();
        let mut nodes = HashMap::new();

        for ent in self.data.entities.keys() {
            let idx = graph.add_node(ent.clone());
            nodes.insert(ent.clone(), idx);
        }

        for rel in &self.data.relations {
            if let (Some(&u), Some(&v)) = (nodes.get(&rel.source), nodes.get(&rel.target)) {
                graph.add_edge(u, v, rel.weight);
            }
        }

        let mut partition = Vec::new();
        let mut visited = HashSet::new();
        for node in graph.node_indices() {
            if visited.contains(&node) {
                continue;
            }
            let mut component = Vec::new();
            let mut stack = vec![node];
            while let Some(n) = stack.pop() {
                if visited.insert(n) {
                    component.push(graph[n].clone());
                    for neighbor in graph.neighbors(n) {
                        stack.push(neighbor);
                    }
                }
            }
            partition.push(component);
        }

        self.data.communities = partition
            .into_iter()
            .enumerate()
            .map(|(i, entities)| {
                let summary = format!(
                    "Community of {} entities including {}.",
                    entities.len(),
                    entities
                        .iter()
                        .take(3)
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                Community {
                    id: i,
                    entities,
                    summary,
                }
            })
            .collect();
    }

    pub fn query(&self, query: &str) -> String {
        let lower_query = query.to_lowercase();
        let query_words: Vec<_> = lower_query.split_whitespace().collect();
        let mut scores = HashMap::new();

        for word in query_words {
            if let Some(chunk_scores) = self.tfidf_index.get(word) {
                for (cid, score) in chunk_scores {
                    *scores.entry(cid).or_insert(0.0) += score;
                }
            }
        }

        let mut sorted_chunks: Vec<_> = scores.into_iter().collect();
        sorted_chunks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if sorted_chunks.is_empty() {
            return "No relevant information found.".to_string();
        }

        let mut response = format!("--- GraphRAG Results ---\n");
        for (cid, _) in sorted_chunks.iter().take(3) {
            if let Some(chunk) = self.data.chunks.get(*cid) {
                response.push_str(&format!(
                    "\n[Chunk ID: {} | Source: {}]\n{}\n",
                    chunk.id, chunk.source, chunk.text
                ));
            }
        }

        for comm in &self.data.communities {
            if comm
                .entities
                .iter()
                .any(|e| query.to_lowercase().contains(&e.to_lowercase()))
            {
                response.push_str(&format!("\n[Community Context: {}]\n", comm.summary));
            }
        }

        response
    }
}
