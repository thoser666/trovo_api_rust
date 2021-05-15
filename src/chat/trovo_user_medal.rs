extern crate image;

use std::collections::HashMap;

pub fn trovo_user_medal()
{
    let blank_image = "data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==".to_String();
    let mut images = HashMap::new();

    let name = "".to_string();
    let image = "".to_string();

    // insert all medals
    images.insert ("streamer_trovo500", "https://img.trovo.live/imgupload/application/20210125_3p78qrxp1fwtrovo_500.png");
    images.insert("xmaschat", "https://img.trovo.live/imgupload/application/20201208_4kpw01ivp1f3x.png");
    images.insert("famhype_deco_ordinary", "https://static.trovo.live/imgupload/application/20200902_22atwe83vh2i3x.png");
    images.insert("recomm_top_tag", "https://static.trovo.live/imgupload/application/20200710_gxd2syiz09a3x.png");
    images.insert("Thanksgiving", "https://static.trovo.live/imgupload/application/20201117_nrupt548ryapp.webp");
    images.insert("warden", "https://static.trovo.live/imgupload/application/20201118_b6g5pkoe2mp2x.png");
    images.insert("XmasHatDrop", "https://img.trovo.live/imgupload/application/20201209_xyuzuimb84.gif");
    images.insert("admins", "https://static.trovo.live/imgupload/application/20200416_7uhhtbuz5cuAdmins.png");
    images.insert("deco_hat", "https://static.trovo.live/imgupload/application/20200618_1rvca870x743x.png");
    images.insert("tag_wish_buff", "https://img.trovo.live/imgupload/application/20201208_kfn7pn2uwr3x.png");
    images.insert("halloweeen_animation", "https://static.trovo.live/imgupload/application/20201016_bfngxjc875web.gif");
    images.insert("Halloween_deco_spin", "https://static.trovo.live/imgupload/application/20201027_vn9lv3xsxr3x.png");
    images.insert("sub_L3", "https://static.trovo.live/imgupload/application/20200429_c52sc2bnrd52x.png");
    images.insert("xmaschat_2", "https://img.trovo.live/imgupload/application/20201208_pu0fvfudl293x.png");
    images.insert("famhype_event_banner", "https://static.trovo.live/imgupload/application/20200825_tj9424s5h0t.jpg");
    images.insert("X'Mas Drop", "https://img.trovo.live/imgupload/shop/20201203_3l2wayxq5i83x.png");
    images.insert("xmaschat_1", "https://img.trovo.live/imgupload/application/20201208_4kpw01ivp1f3x.png");
    images.insert("deco_icon", "https://static.trovo.live/imgupload/application/20200618_ogko8p7fjg3x.png");
    images.insert("sub_L4", "https://static.trovo.live/imgupload/application/20200429_lgxk7tt7nly2x.png");
    images.insert("creator", "https://static.trovo.live/imgupload/application/20200423_yp9vmkduxdBroadcaster.png");
    images.insert("Thanksgiving_Top_hat", "https://static.trovo.live/imgupload/application/20201113_mkuslgoi22b3x.png");
    images.insert("tag_hour_top", "https://img.trovo.live/imgupload/application/20201204_1y927h4qpxyh2x.png");
    images.insert("Xmasicon_2", "https://img.trovo.live/imgupload/application/20201217_kmp8k7nyi53x.png");
    images.insert("mana_rocket_icon", "https://static.trovo.live/imgupload/application/20200904_f7yb2iwgof53x.png");
    images.insert("sub_L1", "https://static.trovo.live/imgupload/application/20200429_xn39uqvlyjx2x.png");
    images.insert("tag_treasure", "https://img.trovo.live/imgupload/application/20201204_t170g1spc2f1.png");
    images.insert("Thanksgiving_hat", "https://static.trovo.live/imgupload/application/20201113_ew99lgp5dlm.png");
    images.insert("famhype_deco_top", "https://static.trovo.live/imgupload/application/20200902_4tisr0ha3os3x.png");
    images.insert("streamer_official", "https://img.trovo.live/imgupload/application/20210125_7wh3aeggwjq2x.png");
    images.insert("tag_weekly_top", "https://img.trovo.live/imgupload/application/20201210_xxsfijknpl3x.png");
    images.insert("X'Mas Permanent", "https://img.trovo.live/imgupload/application/20201217_fgmiy3l9z3.png");
    images.insert("famhype_icon", "https://static.trovo.live/imgupload/application/20200902_m0o621a7sd3x.png");
    images.insert("halloween_icon", "https://static.trovo.live/imgupload/application/20201015_sy5jdu58yfs2x.png");
    images.insert("moderator", "https://static.trovo.live/imgupload/application/20200421_iz479k1142n2x.png");
    images.insert("subscribe_adv", "https://static.trovo.live/imgupload/application/20200429_xi1uidq3lys3x.png");
    images.insert("DefaultTrovo", "https://img.trovo.live/imgupload/application/20201230_hukn0ereb4s3x.png");
    images.insert("event_anim", "https://static.trovo.live/imgupload/application/20200624_1mc9916uq5lc9c0d9eed39d.gif");
    images.insert("Halloween_deco_top3", "https://static.trovo.live/imgupload/application/20201027_o71un8z32eldevilHatTop_big.png");
    images.insert("ThanksgivingEntry", "https://static.trovo.live/imgupload/application/20201111_dlbx9dsvin3x.png");
    images.insert("XmasAvatarIcon", "https://img.trovo.live/imgupload/application/20201217_9se07pb4l1l3x.png");
    images.insert("event_banner", "https://static.trovo.live/imgupload/application/20200618_xqg22xdxwgg3x.png");
    images.insert("Halloween_deco_top3_spin", "https://static.trovo.live/imgupload/application/20201027_npcs9s2w9penchantedHatTop_big.png");
    images.insert("sub_L2", "https://static.trovo.live/imgupload/application/20200429_d8qax5qosul2x.png");
    images.insert("xmaschat_5", "https://img.trovo.live/imgupload/application/20201208_6dyf4uqv2e33x.png");
    images.insert("XmasWishFinish", "https://img.trovo.live/imgupload/application/20201209_8dsfnkkmpal.gif");
    images.insert("sub_L5", "https://static.trovo.live/imgupload/application/20200429_0r3ynjdyottq2x.png");
    images.insert("X'Mas Spin", "https://img.trovo.live/imgupload/application/20201217_fgmiy3l9z3.png");
    images.insert("X'Mas Top", "https://img.trovo.live/imgupload/application/20201217_vsa25z9jkv.png");
    images.insert("Xmasicon_5", "https://img.trovo.live/imgupload/application/20201217_2ffx6cvvzyv.png");
    images.insert("famhype_anim", "https://static.trovo.live/imgupload/application/20200825_59owizb6ffm.webp");
    images.insert("Halloween_deco_ordinary1", "https://static.trovo.live/imgupload/application/20201027_tdfmabu5yy3x.png");
    images.insert("tag_recomm_top", "https://static.trovo.live/imgupload/application/20200708_jucql186p993x.png");
    images.insert("Xmasicon", "https://img.trovo.live/imgupload/application/20201208_nsethnfrmq3x.png");
    images.insert("Xmasicon_1", "https://img.trovo.live/imgupload/application/20201208_nsethnfrmq3x.png");
}
/*
* Replace it with trovo-pendant
 */

// public TrovoUserMedal(String name) {
// this.name = name;
// this.image = IMAGES.getOrDefault(this.name, BLANK_IMAGE);
// }


}