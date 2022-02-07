# Novella Specification
This document should specify, hopefully completely, the various goals of this project and what the final product will hopefully look like.

## Definitions
- Entity: Any single concrete 'thing' in the story--a person, a place, an object,etc...
- Event: Some temporal snapshot with associated entities.
- Scene: A collection of events.
- Act: A collection of scenes.
## Overview
An application that presents a unified environment for the creative writing process—an ‘IDE’ for writers, collecting the features of other disparate applications/manually taking notes into a single location. The full desired features are as follows:

- In depth character/entity creation process, including, but probably not limited to:
    - Basic information (names, ages, etc.)
    - Relation trees (bloods relatives, marriages, friends, enemies, etc.)
    - Incorporation into 'world timeline' (events the character/entity was involved in, data of birth/death, etc.)
    - Development of 'arcs'
    - Arbitrary association of data to the character/entity (photos, videos, links to websites, etc.)
- A GUI built on top of Vulkan
- Creation of Timelines, Events Scenes Arcs and Acts
- A basic text editor
- A language server that can tokenize and provide information about the text to allow for syntax highlighting, completion, etc.
- Ability to render graphs and plots of the character/entity's relationships, timelines and events, etc...
- Ability to write story in 'Progressions', chunks of the story that have an associated ordering, that allow the user to write different parts of the story in not necessarily in chronological order, and then the engine can reorder them later.
- Some sort of version of control. Probably internally use something like git, and present a far more user-friendly interface.

## Goals
By design, Novella is not primarily meant for *writing* stories, but rather to aid in the *development* of stories. The primary goal of Novella is to provide a unified environment for the creative writing process, and to provide a way to visualize the various aspects of the story. Anything related to the publishing/presentation of the story should be done in a separate application, and is considered out of scope (page layout, font selection, etc.)--in other words, if it can be done in Word, do it in Word.
## User flow
The basic flow of usage is the user creates entities (characters, locations, etc…) and fleshes out ideas about said entities by filling in various fields/creating their own fields that describe the entity. These entities are then logged by the engine and when the user next mentioned this entity by name/ID (or any number of aliases) anywhere else in the engine, the engine will highlight quick information/show various other occurrences/jump to definition. These entities are also automatically added to hierarchies/graphs as appropriate. As these stories and description grow, the user can add Arcs/Acts/Scenes to organize the flow of the story. The user can then write a specific part of the story in any order and the engine will automatically organize them together into a chronological whole, this allows in-the-moment ideas/scenes to be quickly written without worrying about sequence or writing everything in a single document in the proper order. The user indicates the order events using  a simple number very analogous to the z-ordering in graphics. 0 is the first event on indefinitely.



## Design
The critical design decision of Novella is running the kernel (the core of the program), as a server, and any number of GUI frontends as clients. The kernel is responsible for maintaining data, any non-transient state and the quintessential logic of the program, while the frontends are responsible for rendering the data and providing user interaction. Currently, the plan is to communicate between the two using Google's Protocol Buffers and gRPC. 
### Kernel
The kernel is primarily written in Rust. As Rust is not an OOP language in the canonical sense, normal inheritence models are not applicable. Instead, it is primarily organized around a data-oriented ECS (Entity-Component-Sysem) model. Similar to how this system might work in a video game, an entity holds a variable number of components which define the things that entity might be able. 
