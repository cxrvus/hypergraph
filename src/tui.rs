use crate::util::vec2::*;

#[derive(Debug)]
struct Id(u32);

#[derive(Debug)]
struct Graph {
	id: Id,
}

#[derive(Debug)]
struct Edge {
	graph: Id,
	lines: Vec<Vec2>,
}

#[derive(Debug)]
struct Node {
	pos: Vec2u,
	size: Vec2u,
	graph: Id,
	content: Item,
}

#[derive(Debug)]
enum Item {
	Label(String), // borderless one line
	Line(String),  // one line
	Text(bool, Id),
	Group(String),
	Graph(bool, Id),
	External(String),
}

#[derive(Debug)]
struct TextItem {
	id: Id,
	content: String,
}

#[derive(Debug, Default)]
enum ItemColor {
	#[default]
	None,
	Red,
	Orange,
	Yellow,
	Green,
	Blue,
	Purple,
	Custom(String),
}

#[derive(Debug)]
struct Database {
	graphs: Vec<Graph>,
	edges: Vec<Edge>,
	nodes: Vec<Node>,
	texts: Vec<TextItem>,
}

#[derive(Debug)]
struct AppState {
	cursor: Vec2,
	view_pos: Vec2,
	view_size: Vec2,
	canvas: Id,
	database: Database,
}

#[derive(Debug, Default)]
enum AppMode {
	#[default]
	Normal,
	InsertNode,
	InsertEdge,
	Visual,
	Move,
	Resize,
	Command,
}
