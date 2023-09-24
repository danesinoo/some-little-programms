pub trait Visitor {
    fn visit_damage_bullet(&mut self);
    fn visit_damage_slow(&mut self);
    fn visit_damage_wave(&mut self);

    fn visit_sample(&mut self);
    fn visit_three_column(&mut self);
    fn visit_double_life(&mut self);
    fn visit_three_row(&mut self);
    fn visit_slow_down(&mut self);

    fn visit_enemy(&mut self);
    fn visit_enemy_defense(&mut self);
    fn visit_enemy_big(&mut self);

    fn visit_tool_weapon(&mut self);
    fn visit_tool_armor(&mut self);
    fn visit_tool_boots(&mut self);
    fn visit_tool_ring(&mut self);
}

pub trait Visitable {
    fn accept(&self, visitor: &mut dyn Visitor);
}
