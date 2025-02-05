# The Bat Game

## Title Ideas:
- Flappy Bat

## Todos:
### Graphics
- [x] Draw Bat in Center
- [x] Bat Sprite scales to size of window
    - [x] Set Window to 1280x800, non-Resizeable
    - [x] Set Window to Fullsceen on Release, Windowed on on Debug
- [x] Background
- [x] Flap Animation
- [ ] World Darkens as time progresses

### Gameplay
- [ ] Receive Inputs
    - [x] Controller
    - [x] Keyboard
    - [x] Mouse
    - [x] Exclude Input when Window not focused
    - [ ] Menu Inputs
- [x] Bat Movement
    - [x] Gravity
    - [x] Vertical Flapping
    - [x] Horizontal Movement
    - [x] Tune movement so it feels just right
    - [ ] Extra Gravity to bring bat down from above screen
- [x] Pause
- [x] Reset
- [ ] Game Over when Bat hits bottom
- [ ] Enemies Spawn
- [ ] Enemies Despawn when hit
- [ ] Score increased when enemy eaten
- [ ] Reset at level end

### UI
- [ ] Score Displayed on screen
- [ ] Prompt to Start game
- [ ] Pause Menu
    - [ ] Continue 
    - [ ] Restart
    - [ ] Quit

### Sound
- [x] Flap Noise
- [ ] Crunch sound when enemies eaten
- [x] Screecth Noise 

### Development
- [x] Create basic github workflow
- [ ] Automated Tests
    - [ ] Test Asset Loading
- [ ] Logging
- [ ] Add Diagnostics

### Issues
- [x] PowerA Controller thnks "Y" is West
    (resolved via environment variable. Will eventually be fixed with update to gilrs

## Enemy ideas
- Firefly
- Dragonfly
- Moth
- Cicada
- Gnat
- Mosquito
- Grasshopper (Jumps from buttom maybe)
