# Tiro

A Task Graph written in Rust

<!-- [![Crates.io][crates-badge]][crates-url] -->
[![MIT licensed][mit-badge]][mit-url]
![Rust](https://github.com/UsairimIsani/tiro/workflows/Rust/badge.svg)
![Discord](https://img.shields.io/discord/795616551910113280)

<!-- [crates-badge]: https://img.shields.io/crates/v/tiro.svg -->
<!-- [crates-url]: https://crates.io/crates/tiro -->

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/UsairimIsani/tiro/blob/main/LICENSE

Create a ***TaskGraph***

**Register Task in any order**

```rust
let task_graph = task_graph.register("task1", task);
let task_graph = task_graph.register("task1", task);
let task_graph = task_graph.register("task4", task);
let task_graph = task_graph.register("task3", task);
let task_graph = task_graph.register("task2", task);
let task_graph = task_graph.register("task5", sub_task_graph);  // Using Trait we will also be able to register a `TaskGraph` as a task  
let task_graph = task_graph.register("task6", task);
```

Will Also support the Builder Pattern. Which would enable Iterator folding

```rust
task_graph.register("task1", task)
	.register("task1", task)
	.register("task4", task)
	.register("task3", task)
	.register("task2", task)
	.register("task5", 
			sub_task_graph
					.register("task7",task) // Using Trait we will also be able to register a `TaskGraph` as a task  
					.register("task8",task) 
					...	
	)  
.register("task6", task);
```

**Describe Dependency Tree in a Macro.**

```rust
create_dependencies!(
task_graph,
	task6 >> task5, // Can have sub TaskGraphs
	task5 >> ["task3","task4"],
	task4 >> task3,
	task3 >> task2,
	task2 >> task1
)
```

Creates a Hierarchy like 

```markdown
// task_graph
task1 >> task2 >> task3   >> task5 >> task6
								\>> task4 /

// sub_task_graph (task5)
task7 >> task8   >> task10 >> 
			\>> task9 /

// Complete TaskGraph
task1 >> task2 >> task3   >> task7 >> task8 >> task10 >> task6
								\>> task4 /         \>> task9 /  
```







---
#### The Whole process of Building this Library has been documented on Notion
### [Notion](https://www.notion.so/Project-Tiro-Task-Graph-Library-in-Rust-5f6e916f987b490386620a9b30d3616c)