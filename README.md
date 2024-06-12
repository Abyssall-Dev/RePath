# RePath

RePath is a fast and efficient pathfinding library specifically designed for MMO game servers. It leverages the A* algorithm and bidirectional search to provide rapid and precise pathfinding solutions, essential for managing large numbers of NPCs and players in real-time environments.

RePath was developed for [Respark](https://playrespark.com/), an upcoming open world MMO shooter. Respark combines intense combat, strategic gameplay, and a vast, dynamic world to explore. Join our community on [Discord](https://discord.gg/8qzSGyekVJ) to stay updated with the latest news and development progress.

## Description

RePath was developed to address the need for high-performance pathfinding in a game server written for the upcoming open world MMO shooter called [Respark](https://playrespark.com/). Given the complexity and size of game worlds, pathfinding can be a significant bottleneck. RePath optimizes this process through a combination of precomputation, caching, and advanced search algorithms, ensuring quick and accurate pathfinding even in demanding scenarios.

### How It Works

- **A\* Algorithm**: A widely-used pathfinding algorithm known for its efficiency and accuracy in finding the shortest path.
- **Bidirectional Search**: Enhances the A* algorithm by searching from both the start and goal nodes simultaneously, reducing the search space and improving speed.
- **Precomputation**: Paths are precomputed and cached to provide near-instantaneous results right from the start.
- **LRU Cache**: Least Recently Used (LRU) cache ensures efficient memory usage by storing only the most recently accessed paths.

### Why It's Fast

RePath's speed comes from its combination of precomputation, efficient search algorithms, and intelligent caching. By precomputing paths and storing them in an LRU cache, RePath can quickly return results for common pathfinding queries without recalculating. The bidirectional search further reduces the search space, making the pathfinding process faster and more efficient.

## Features

- **A\* Pathfinding Algorithm**: Efficient and accurate pathfinding.
- **Bidirectional Search**: Faster search by reducing the search space.
- **Precomputation**: Quickly precomputes random paths in parallel using [Rayon](https://crates.io/crates/rayon) and stores them in a cache.
- **LRU Cache**: Efficient memory usage and quick access to recent paths.
- **Scalable**: Handles large game worlds and numerous NPCs.

## Usage

### Adding RePath to Your Project

Add RePath to your `Cargo.toml`:

```toml
[dependencies]
repath = "0.0.4"
```

Make sure you have the OBJ file containing the navmesh in the same directory as your project.

Then use it in your project:

```rust
use repath::{RePathfinder, settings::RePathSettings};

fn main() {
    // Create a new RePathSettings instance with custom settings
    let settings = RePathSettings {
        navmesh_filename: "navmesh_varied.obj".to_string(), // Path to the navmesh file in Wavefront OBJ format
        precompute_radius: 25.0, // Higher this value, the longer it takes to precompute paths but faster pathfinding for long distances
        total_precompute_pairs: 1000, // Higher this value, the longer it takes to precompute paths but faster pathfinding
        cache_capacity: 1000, // Higher this value, the more paths can be stored in cache but more memory usage
        use_precomputed_cache: true, // Set to false to disable precomputation of paths
    };

    // Create a new RePathfinder instance
    let pathfinder = RePathfinder::new("navmesh_varied.obj", settings);

    // Define start and end coordinates for pathfinding
    let start_coords = (0.0, 0.0, 0.0);
    let end_coords = (10.0, 10.0, 10.0);

    // Find a path from start to end coordinates
    if let Some(path) = pathfinder.find_path(start_coords, end_coords) {
        println!("Found path: {:?}", path);
    } else {
        println!("No path found.");
    }
}
```

### License

RePath is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
