use crate::enemies::list_enemies;

#[test]
fn partial_enemy_diff() {
    let enemies = list_enemies();
    let illuminant_bat = enemies.iter().find(|e| e.id == 137).unwrap();
    let dreamer_ghoul = enemies.iter().find(|e| e.id == 527).unwrap();
    let bad_diff = illuminant_bat.diff(&dreamer_ghoul);
    assert!(bad_diff.environments.missing);

    let vampire = enemies.iter().find(|e| e.id == 159).unwrap();
    let reaper = enemies.iter().find(|e| e.id == 253).unwrap();
    let good_diff = vampire.diff(&reaper);
    assert!(good_diff.environments.missing);
}
