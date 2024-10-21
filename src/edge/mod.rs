use {
	smash::{
		app::{
			ArticleOperationTarget,
			AttackHeight,
			lua_bind::*,
			sv_animcmd
		},
		lib::lua_const::*,
		lua2cpp::L2CAgentBase,
		phx::*
	},
	smash_script::*,
	smashline::*
};

   
   unsafe extern "C" fn game_AttackAirHi(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    sv_animcmd::frame(agent.lua_state_agent, 5.0);
}
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);

}

sv_animcmd::frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    sv_animcmd::frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
         //  macros::ATTACK(agent, [don't edit this...ever]
         //0, [id of hitbox]
         // 0, [part; leave this alone]
         // Hash40::new("swordl1"), [bone]
         // 7.0, [damage]
         // 106, [angle]
         // 96, 0, 55, [knockback growth, fixed knockback, base knockback]
         // 4.0, [size]
         // 0.5, 0.0, 3.0, [position of hitbox relative to bone *x, y, z*]
         // None, None, None, [how much a hitbox stretches in terms of a 3 dimensional axis; if used, include "some({whatever float value})"; if not used, always write "None"]
         // 0.8, [hitlag]
         // 1.0, [sdi]
         // *ATTACK_SETOFF_KIND_ON, [clang rebound]
         // *ATTACK_LR_CHECK_F, [facing restriction]
         // false, [set weight; knockback is affected by weight; used in conjunction with fixed knockback]
         // 0,[shield damage; add int value to increase how much shield damage a move does, eg Marcina Shield Breaker has a value of 2]
         // 0.0, [trip chance]
         // 0, [rehit; decides how long until another hitbox can hit an opponent]
         // false, [reflectable]
         // false,[absorbable]
         // false, [flinchless]
         // false,[disable hitlag]
         // true, [direct hitbox; false if hitbox is from a weapon]
         // *COLLISION_SITUATION_MASK_GA, [ground or air]
         // *COLLISION_CATEGORY_MASK_ALL, [hitbits]
         // *COLLISION_PART_MASK_ALL, [collision part]
         // false, [friendly fire]
         // Hash40::new("collision_attr_cutup"), [effect; involves stun, burial, freeze, etc]
         // *ATTACK_SOUND_LEVEL_M, [sfx level]
         // *COLLISION_SOUND_ATTR_CUTUP, [sfx type]
         // *ATTACK_REGION_SWORD); [type; used with spirits when buffing moves involving punches, kicks, etc]
        macros::ATTACK(agent, 0, 0, Hash40::new("swordl1"), 7.0, 106, 96, 0, 55, 4.0, 0.5, 0.0, 3.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("swordl1"), 11.0, 106, 96, 0, 55, 4.0, 7.0, 0.0, 3.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("swordl1"), 11.0, 106, 96, 0, 55, 4.0, 14.0, 0.0, 3.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("swordl1"), 8.5, 106, 96, 0, 55, 4.0, 21.0, 0.0, 3.0, Some(1.0), Some(0.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("swordl1"), 7.0, 74, 96, 0, 55, 4.0, 0.5, 0.0, 3.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("swordl1"), 11.0, 74, 96, 0, 55, 4.0, 7.0, 0.0, 3.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("swordl1"), 11.0, 74, 96, 0, 55, 4.0, 14.0, 0.0, 3.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("swordl1"), 8.5, 74, 96, 0, 55, 4.0, 21.0, 0.0, 3.0, Some(1.0), Some(0.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

pub fn install(){

    Agent::new("edge")
        .game_acmd("game_AttackAirHi", edge_game_AttackAirHi, Priority::Default)
        .install();
}
