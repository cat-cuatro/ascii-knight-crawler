# ascii-knight-crawler
An RPG-style simulation-themed Rust class project.

## Early Vision
A procedural dungeon crawler using text. I'll be using RPG elements to drive design choices and keep the application visually simple. 

- Character stats
- Item(s) and associated benefits
- Random enemies
- Random environments
- Health statuses
- etc ..

# Final Submission
For my final submission, I implemented Bart's suggestion to have the character fully autonomous.
### What has been built
Essentially, I've been a small simulation game. I didn't get to all of my ideas; like many projects, my ambition was quickly put in check! However, I can create a character and have it controlled by a randomly acting computer agent. While I didn't implement Q-Learning like I intended once I pivoted, the architectural design is set up in a way that *any* agent could be coded into `computer.rs` as long as it returns a valid action to `computer_play`.
I do everything on the command line by using ANSI escapes to clear the terminal and re-print the current state space. The application isn't bug-free, but achieves the purpose of simulating the behavior of a totally random behaving knight (or warrior) character wandering about in a 10x10 square. When the character moves onto a tile already occupied by an enemy, it instead behaves as an attack. The simulation ends when the character runs out of health or stamina. As the character defeats enemies, its survial score increments. Right now the survival score is simply tethered to how many enemies are defeated. As the game runs, difficulty increases corresponding to the number of game ticks that have passed.
### How it works
In total I have 8 source files with distinct purposes.
- `computer.rs` contains any 'agents' that can play the game.
- `character.rs` contains the character creation, and all of its movement abilities, combat attributes, and has an `archetype`.
- `archetype.rs` contains the base stats associated with any given `archetype` and handles the definition of `archetypes`. (technically I should've called this classes, but I was already too far along by the time I realized)
- `monster_archetype.rs` contains monster definitions in the same way `archetype.rs` does.
- `hostile_enemy.rs` contains the implementation of NPC enemies. Specifically things like checking for life, status, loot drops, and status modifiers like `character.rs`. A `hostile_enemy` *has* an enemy archetype from `monster_archetype.rs`, which is where it inherits its attributes.
- `world_tiles.rs` serves as a microcosm to the overworld. Each tile in the overworld has a symbol indicating the presence (or lack of) an NPC or character and a position. Tiles can update their content to reflect the overworld.
- `overworld.rs` is the map and simulation engine. I store the complex logic in here. The overworld initiates 'new events' which in my case are just monster spawns, but can be extended to items or anything a `world tile` is capable of doing. The overworld resolves combat interactions, difficulty scaling, reward systems, and visual updates. The overworld also has a game tick system.
- `main.rs` handles the game loop. Input actions from the computer are passed through here and those choices are fed into the `overworld` where it advances to the next stage of the simulation. In a ways it acts as a controller to the overall simulation, but these functions could easily be broken into a new crate. I've left them for now.
### What didn't work
I should've known from the beginning that trying to organically develop the architecture of the simulation as I wrote it was a bad idea. I correctly created the archetypes, characters, and smaller units of function first but when it came time to design how they interacted it became very difficult. I tried using boolean flags to dictate combat and pass them around in complicated ways but it clearly wasn't working which is what led me to my overworld design. I also don't like my `Direction` enum design; when it works it's nice, but it really feels more like I'm implementing a struct for code distinction rather than adding value. I wanted to try using enums in Rust which is why I did it.
### Learnings
I learned a great deal about Rust. I struggled quite a bit with the struct-impl concepts and how to correctly pass data around as well as the mutable ref, ref, copy, clone distinctions. In general it was a difficult start, after I got the overworld working and had all of these complex interactions taking place I started to see the Rust vision.
If I had to do this project again I would try to first architect it. There are a lot of opportunities to use things like config files, better usage of the `Direction` enum to deduplicate my movement methods for `character.rs` and `hostile_enemy.rs`. I could have also used more data structures to handle combat sequences like queues. In general there are a lot of improvements I could make and opportunity to expand this program in an enriching way.


## AI Notice
I used Claude Haiku for debugging assistance, and for driving a deeper understanding of nuanced Rust concepts like lifetimes, enums, and mutable refs. Code, logic, and design are my own.