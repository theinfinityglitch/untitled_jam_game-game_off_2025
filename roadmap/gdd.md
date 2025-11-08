### ⚠️ Warning: This document is under constant modifications

# Metadata

- **Game Genre:** Souls-Like; Metroidvania.

# Core loop

- **Observation Phase:** Use your vision to assess the room - creature positions, patrol routes, environmental objects

- **Stealth Execution:** Move carefully, manage sound levels, use distractions

- **Tactical Engagement:** Choose when to fight, when to avoid, when to manipulate

# Key Mechanics

## Sound Management System

- **Walking** - Minimal sound, safe for most situations

- **Running** - Loud, attracts nearby creatures within ~5-7 tiles

- **Landing from jumps** - Creates sound burst based on height

- **Attacking** - Weapon-dependent sound radius

- **Interacting** - Opening doors, chests creates localized sound

## Environmental Interactions

- **Throwable objects** - Create distraction sounds at target location

- **Different surfaces** - Stone (louder), grass (quieter), water (splashing sounds)

- **Breakable objects** - Can create loud noises that alert entire area

## Creature AI Behaviors

- States: Sleeping → Alert (head tilting) → Investigating (move to sound) → Aggressive

- Creatures track last known sound location

- They can lose interest if no further sounds

- Different creatures have different hearing sensitivity

# Player Arsenal

- **Starting dagger** - Quiet but short range

- **Short bow** - Silent ranged option (limited arrows). Found early

- **Throwable rocks** - Unlimited distractions. Found around the map

- **Healing items** - Require standing still for 2-3 seconds (makes minimal sound)

# Art/Tech

## Pipeline

- **Primary programming language:** Rust

- **Engine/Framework:** [Bevy](https://github.com/bevyengine/bevy)

- **Third party crates:** [bevy_rapier_2d](https://github.com/dimforge/bevy_rapier), [bevy_ecs_ldtk](https://github.com/Trouv/bevy_ecs_ldtk), [bevy_egui](https://github.com/vladbat00/bevy_egui)

- **Pixel art tool:** [Pixelorama](https://github.com/Orama-Interactive/Pixelorama)

- **Level editor:** [LDtk](https://github.com/deepnight/ldtk)
