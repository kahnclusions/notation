use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub mod block;

//Pages

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageProps {
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageBlock {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub children: Vec<Block>,
    pub props: PageProps,
}

// Text

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextProps {
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextBlock {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub children: Vec<Block>,
    pub props: TextProps,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DummyBlock {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub children: Vec<Block>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Block {
    #[serde(rename = "page")]
    Page(PageBlock),
    #[serde(rename = "text")]
    Text(TextBlock),
    #[serde(rename = "empty")]
    Dummy(DummyBlock),
}

impl Block {
    pub fn id(&self) -> Uuid {
        match self {
            Self::Page(blk) => blk.id.clone(),
            Self::Text(blk) => blk.id.clone(),
            Self::Dummy(blk) => blk.id.clone(),
        }
    }

    pub fn set_children(mut self, children: Vec<Self>) -> Self {
        match self {
            Self::Page(ref mut blk) => blk.children = children,
            Self::Text(ref mut blk) => blk.children = children,
            Self::Dummy(_) => {}
        }
        self
    }
}

#[cfg(feature = "ssr")]
pub fn build_page_tree(page_id: Uuid, blocks: &Vec<block::SqlBlock>) -> anyhow::Result<PageBlock> {
    use std::collections::HashMap;

    let mut children: HashMap<Uuid, Vec<Block>> = HashMap::new();
    let id_str = page_id.to_string();
    let page: Option<PageBlock> = blocks.iter().find_map(|blk| {
        if blk.id == id_str {
            let block: Block = blk.to_owned().into();
            match block {
                Block::Page(page_block) => Some(page_block),
                _ => None,
            }
        } else {
            None
        }
    });

    let Some(page) = page else {
        tracing::info!(blocks = ?blocks);
        anyhow::bail!("Oops, no page in the blocks")
    };

    // Sort blocks according to parent
    for block in blocks.into_iter() {
        match block.parent_id.clone() {
            None => {}
            Some(parent_id) => {
                let parent_id = Uuid::from_str(parent_id.as_str())?;
                match children.get_mut(&parent_id) {
                    Some(current) => {
                        current.push(block.to_owned().into());
                    }
                    None => {
                        children.insert(parent_id, vec![block.to_owned().into()]);
                    }
                }
            }
        }
    }

    fn process_children(block: Block, children: &HashMap<Uuid, Vec<Block>>) -> Block {
        let Some(block_children) = children.get(&block.id()) else {
            return block;
        };

        let mut new_children = Vec::new();
        for child in block_children.into_iter() {
            let child = process_children(child.clone(), children);
            new_children.push(child);
        }
        block.set_children(new_children)
    }

    match process_children(Block::Page(page), &children) {
        Block::Page(page_block) => Ok(page_block),
        _ => Err(anyhow::anyhow!(
            "Oops, didn't return a page block at the root"
        )),
    }
}

#[cfg(feature = "ssr")]
impl From<block::SqlBlock> for Block {
    fn from(value: block::SqlBlock) -> Self {
        let id = Uuid::from_str(value.id.as_str()).unwrap();
        let parent_id = value
            .parent_id
            .map(|id| Uuid::from_str(id.as_str()).unwrap());
        match value.kind.as_str() {
            "page" => Block::Page(PageBlock {
                id,
                parent_id,
                children: Vec::new(),
                props: serde_json::from_str(value.props.as_str()).unwrap(),
            }),
            "text" => Block::Text(TextBlock {
                id,
                parent_id,
                children: Vec::new(),
                props: serde_json::from_str(value.props.as_str()).unwrap(),
            }),
            _ => Block::Dummy(DummyBlock {
                id,
                parent_id,
                children: Vec::new(),
            }),
        }
    }
}
