# rusty tetris
a tetris clone written in rust

## ideas:
```
pub struct pos{
    int:x,
    int:y,
}

pub struct Block{
    pub data:[pos;3],
    (pub SDL_IMAGE),
    pub speed: int / pub speed_up:bool,
}
impl Block{
    fn update(placed_blocks:[[bool;n];m] / game:Game);
    fn draw();
    fn rotate();
    fn check_collision(placed_blocks);
}

pub struct Game{
    pub current_block:Block,
    pub keyboard_heandler,
    pub placed_blocks:[[bool;n];m],
}

impl Game{
    fn handle_keyboard_input();
    fn draw();
    fn update();
}

```
