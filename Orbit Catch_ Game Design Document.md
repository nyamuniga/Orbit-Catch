# **Orbit Catch: Game Design Document**

## **1\. Executive Summary**

Orbit Catch is a rhythm and timing-based game that embodies the "low floor, high ceiling" design philosophy. Players manipulate gravity to catch incoming projectiles into stable orbits. The game scales from a simple sensory experience to a highly complex orbital management puzzle.

## **2\. Core Gameplay Mechanics**

* **The Core Input:** A single screen tap pulses the central "Sun", emitting a wave of gravitational force.  
* **The Moons:** Small geometric shapes fly in from the edges of the screen at varying speeds and trajectories.  
* **The Catch:** Timing the gravitational pulse exactly when a Moon crosses a designated orbital ring captures it.  
* **The Failure State:** If an un-captured Moon hits the Sun, or if two orbiting Moons collide due to intersecting orbital paths, the game ends.

## **3\. Visual and Aesthetic Identity**

The design language relies on a modern, minimalist aesthetic. The user interface eschews clutter in favor of clean typography and strict geometric structures. The visual representation leans towards a high-end, polished style, prioritizing deep contrasting colors and smooth orbital trails over chaotic particle effects.

## **4\. Difficulty Scaling and Progression**

| Stage | Projectile Speed | Orbital Complexity | Target Audience Experience   |
| :---- | :---- | :---- | :---- |
| 1\. Nursery | Slow, linear | Single orbital ring | Toddler: Enjoys the musical chime of a successful catch. |
| 2\. System | Moderate, curved | Up to 3 concentric rings | Older Child: Pattern recognition and basic timing. |
| 3\. Galaxy | Fast, erratic | Intersecting elliptical orbits | Adult: Frantic micro-management and trajectory prediction. |

## **5\. Technical Considerations**

// Pseudo-code for Orbital Catch Logic  
function checkCatch(moon, sunPulse) {  
    if (distance(moon, sunPulse.center) \== orbitalRing.radius) {  
        moon.velocity \= calculateOrbitalVelocity(moon.mass, sunPulse.force);  
        moon.state \= ORBITING;  
        playChime(moon.resonantFrequency);  
        addScore(calculateCombo());  
    } else {  
        triggerDeflection(moon);  
    }  
}  
