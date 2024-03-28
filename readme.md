# Bevy Castle Fight

This game takes inspiration from the Warcraft III Custom Map called Castle Fight.
The basics of the game are that players place buildings that auto-spawn units.

The units will move towards the enemy base, which they will try to destroy.
They will fight enemy units they encounter along the way.

Each team tries to build the correct units in order to overcome their enemies' forces.

## Setting up the code base

### Requirements

- Rust
- Git & Git LFS
- LDtk (optional) - For creating and altering maps

### Setup

- Install Rust, Git and Git LFS before cloning the repository.
- Clone the repository and initialise Git LFS in the repo folder.
- Run the command `cargo run` to check that everything works.

### Problems

#### Problems with Git LFS or missing files

If encountering problems with missing files like PNG files or errors during cloning or pulling the repo
you can try and use Github Desktop (or another git client), which might help with correct Git LFS setup.

## External tools

### LDtk

LDtk is a free and open-source 2D level editor. In this project we use it to create maps, setting up entities and more. 

## Notes about the pipelines

The CI and release actions are modified versions of the examples in the Bevy CI template repo:

https://github.com/bevyengine/bevy_github_ci_template

A few changes were necessary to make the actions run.

In the CI pipeline, Clippy needs to have added permissions to run:

`permissions: write-all`

This should be at the top level of the job.

Furthermore, each job that uploads a release in the release pipeline also needs added permissions to run:

`permissions: write-all`

This should be at the top level of the job.

This has been included in the template.
