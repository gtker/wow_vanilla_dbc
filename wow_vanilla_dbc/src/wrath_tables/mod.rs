pub mod achievement;
pub mod achievement_category;
pub mod achievement_criteria;
pub mod animation_data;
pub mod area_group;
pub mod area_poi;
pub mod area_table;
pub mod area_trigger;
pub mod attack_anim_kits;
pub mod attack_anim_types;
pub mod auction_house;
pub mod bank_bag_slot_prices;
pub mod banned_addons;
pub mod barber_shop_style;
pub mod battlemaster_list;
pub mod camera_shakes;
pub mod cfg_categories;
pub mod cfg_configs;
pub mod char_base_info;
pub mod char_hair_geosets;
pub mod char_hair_textures;
pub mod char_sections;
pub mod char_start_outfit;
pub mod char_titles;
pub mod char_variations;
pub mod character_facial_hair_styles;
pub mod chat_channels;
pub mod chat_profanity;
pub mod chr_classes;
pub mod chr_races;
pub mod cinematic_camera;
pub mod cinematic_sequences;
pub mod creature_display_info;
pub mod creature_display_info_extra;
pub mod creature_family;
pub mod creature_model_data;
pub mod creature_movement_info;
pub mod creature_sound_data;
pub mod creature_spell_data;
pub mod creature_type;
pub mod currency_category;
pub mod currency_types;
pub mod dance_moves;
pub mod death_thud_lookups;
pub mod declined_word;
pub mod declined_word_cases;
pub mod destructible_model_data;
pub mod dungeon_encounter;
pub mod dungeon_map;
pub mod dungeon_map_chunk;
pub mod durability_costs;
pub mod durability_quality;
pub mod emotes;
pub mod emotes_text;
pub mod emotes_text_data;
pub mod emotes_text_sound;
pub mod environmental_damage;
pub mod exhaustion;
pub mod faction;
pub mod faction_group;
pub mod faction_template;
pub mod file_data;
pub mod footprint_textures;
pub mod footstep_terrain_lookup;
pub mod game_object_art_kit;
pub mod game_object_display_info;
pub mod game_tables;
pub mod game_tips;
pub mod gem_properties;
pub mod glyph_properties;
pub mod glyph_slot;
pub mod gm_survey_answers;
pub mod gm_survey_current_survey;
pub mod gm_survey_questions;
pub mod gm_survey_surveys;
pub mod gm_ticket_category;
pub mod ground_effect_doodad;
pub mod ground_effect_texture;
pub mod gt_barber_shop_cost_base;
pub mod gt_chance_to_melee_crit;
pub mod gt_chance_to_melee_crit_base;
pub mod gt_chance_to_spell_crit;
pub mod gt_chance_to_spell_crit_base;
pub mod gt_combat_ratings;
pub mod gt_npc_mana_cost_scaler;
pub mod gt_oct_class_combat_rating_scalar;
pub mod gt_oct_regen_hp;
pub mod gt_oct_regen_mp;
pub mod gt_regen_hp_per_spt;
pub mod gt_regen_mp_per_spt;
pub mod helmet_geoset_vis_data;
pub mod holiday_descriptions;
pub mod holiday_names;
pub mod holidays;
pub mod item;
pub mod item_bag_family;
pub mod item_class;
pub mod item_cond_ext_costs;
pub mod item_display_info;
pub mod item_extended_cost;
pub mod item_group_sounds;
pub mod item_limit_category;
pub mod item_pet_food;
pub mod item_purchase_group;
pub mod item_random_properties;
pub mod item_random_suffix;
pub mod item_set;
pub mod item_sub_class;
pub mod item_sub_class_mask;
pub mod item_visual_effects;
pub mod item_visuals;
pub mod language_words;
pub mod languages;
pub mod lfg_dungeon_expansion;
pub mod lfg_dungeon_group;
pub mod lfg_dungeons;
pub mod light;
pub mod light_float_band;
pub mod light_int_band;
pub mod light_params;
pub mod light_skybox;
pub mod liquid_material;
pub mod liquid_type;
pub mod loading_screen_taxi_splines;
pub mod loading_screens;
pub mod lock;
pub mod lock_type;
pub mod mail_template;
pub mod map;
pub mod map_difficulty;
pub mod material;
pub mod movie;
pub mod movie_file_data;
pub mod movie_variation;
pub mod name_gen;
pub mod names_profanity;
pub mod names_reserved;
pub mod npc_sounds;
pub mod object_effect;
pub mod object_effect_group;
pub mod object_effect_modifier;
pub mod object_effect_package;
pub mod object_effect_package_elem;
pub mod override_spell_data;
pub mod package;
pub mod page_text_material;
pub mod paper_doll_item_frame;
pub mod particle_color;
pub mod pet_personality;
pub mod petition_type;
pub mod power_display;
pub mod pvp_difficulty;
pub mod quest_faction_reward;
pub mod quest_info;
pub mod quest_sort;
pub mod quest_xp;
pub mod rand_prop_points;
pub mod resistances;
pub mod scaling_stat_distribution;
pub mod scaling_stat_values;
pub mod screen_effect;
pub mod server_messages;
pub mod sheathe_sound_lookups;
pub mod skill_costs_data;
pub mod skill_line;
pub mod skill_line_ability;
pub mod skill_line_category;
pub mod skill_race_class_info;
pub mod skill_tiers;
pub mod sound_ambience;
pub mod sound_emitters;
pub mod sound_entries;
pub mod sound_entries_advanced;
pub mod sound_filter;
pub mod sound_filter_elem;
pub mod sound_provider_preferences;
pub mod sound_sample_preferences;
pub mod sound_water_type;
pub mod spam_messages;
pub mod spell;
pub mod spell_cast_times;
pub mod spell_category;
pub mod spell_chain_effects;
pub mod spell_description_variables;
pub mod spell_difficulty;
pub mod spell_dispel_type;
pub mod spell_duration;
pub mod spell_effect_camera_shakes;
pub mod spell_focus_object;
pub mod spell_icon;
pub mod spell_item_enchantment;
pub mod spell_item_enchantment_condition;
pub mod spell_mechanic;
pub mod spell_missile;
pub mod spell_missile_motion;
pub mod spell_radius;
pub mod spell_range;
pub mod spell_rune_cost;
pub mod spell_shapeshift_form;
pub mod spell_visual;
pub mod spell_visual_effect_name;
pub mod spell_visual_kit;
pub mod spell_visual_kit_area_model;
pub mod spell_visual_kit_model_attach;
pub mod spell_visual_precast_transitions;
pub mod stable_slot_prices;
pub mod startup_strings;
pub mod stationery;
pub mod string_lookups;
pub mod summon_properties;
pub mod talent;
pub mod talent_tab;
pub mod taxi_nodes;
pub mod taxi_path;
pub mod taxi_path_node;
pub mod team_contribution_points;
pub mod terrain_type;
pub mod terrain_type_sounds;
pub mod totem_category;
pub mod transport_animation;
pub mod transport_physics;
pub mod transport_rotation;
pub mod ui_sound_lookups;
pub mod unit_blood;
pub mod unit_blood_levels;
pub mod vehicle;
pub mod vehicle_seat;
pub mod vehicle_ui_ind_seat;
pub mod vehicle_ui_indicator;
pub mod video_hardware;
pub mod vocal_ui_sounds;
pub mod weapon_impact_sounds;
pub mod weapon_swing_sounds2;
pub mod weather;
pub mod wmo_area_table;
pub mod world_chunk_sounds;
pub mod world_map_area;
pub mod world_map_continent;
pub mod world_map_overlay;
pub mod world_map_transforms;
pub mod world_safe_locs;
pub mod world_state_ui;
pub mod world_state_zone_sounds;
pub mod wow_error_strings;
pub mod zone_intro_music_table;
pub mod zone_music;
