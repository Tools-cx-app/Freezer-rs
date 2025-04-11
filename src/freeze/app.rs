use std::{
    collections::{HashMap, HashSet},
    process::Command,
};

use anyhow::{Context, Result};
use regex::Regex;

lazy_static::lazy_static! {
    static ref APP_REGEX: Regex = Regex::new(r".*\{([^/]+)/").unwrap();
}
static WHITE_LIST: [&str; 365] = [
    "com.xiaomi.mibrain.speech",  // 系统语音引擎
    "com.xiaomi.scanner",         // 小爱视觉
    "com.xiaomi.xmsf",            // Push
    "com.xiaomi.xmsfkeeper",      // Push
    "com.xiaomi.misettings",      // 设置
    "com.xiaomi.barrage",         // 弹幕通知
    "com xiaomi.aireco",          // 小爱建议
    "com.xiaomi.account",         // 小米账号
    "com.miui.notes",             // 笔记  冻结会导致系统侧边栏卡住
    "com.miui.calculator",        // 计算器
    "com.miui.compass",           // 指南针
    "com.miui.mediaeditor",       // 相册编辑
    "com.miui.personalassistant", // 个人助理
    "com.miui.vipservice",        // 我的服务
    "com.miui.systemAdSolution",  // 智能助理 广告相关 冻结会导致酷安等应用卡顿
    "com.miui.home",
    "com.miui.carlink",
    "com.miui.packageinstaller",    // 安装包管理
    "com.miui.accessibility",       // 小米无障碍
    "com.miui.core",                // MIUI SDK
    "com.miui.privacycomputing",    // MIUI Privacy Components
    "com.miui.securityadd",         // 系统服务组件
    "com.miui.securityinputmethod", // 小米安全键盘
    "com.miui.system",
    "com.miui.vpnsdkmanager",                // MiuiVpnSdkManager
    "com.mfashiongallery.emag",              // 小米画报
    "com.huawei.hwid",                       // HMS core服务
    "com.merxury.blocker",                   // Blocker
    "com.wpengapp.lightstart",               // 轻启动
    "com.sevtinge.hyperceiler",              // HyperCeiler
    "org.lsposed.manager",                   // LSPosed
    "name.monwf.customiuizer",               // 米客 原版
    "name.mikanoshi.customiuizer",           // 米客
    "com.android.vending",                   // Play 商店
    "org.meowcat.xposed.mipush",             // 小米推送框架增强
    "top.trumeet.mipush",                    // 小米推送服务
    "one.yufz.hmspush",                      // HMSPush服务
    "app.lawnchair",                         // Lawnchair
    "com.microsoft.launcher",                // 微软桌面
    "com.teslacoilsw.launcher",              // Nova Launcher
    "com.hola.launcher",                     // Hola桌面
    "com.transsion.XOSLauncher",             // XOS桌面
    "com.mi.android.globallauncher",         // POCO桌面
    "com.gau.go.launcherex",                 // GO桌面
    "bitpit.launcher",                       // Niagara Launcher
    "com.google.android.apps.nexuslauncher", // pixel 桌面
    "com.oppo.launcher",
    "top.canyie.dreamland.manager",         // Dreamland
    "com.coloros.packageinstaller",         // 安装包管理
    "com.oplus.packageinstaller",           // 安装包管理
    "com.iqoo.packageinstaller",            // 安装包管理
    "com.vivo.packageinstaller",            // 安装包管理
    "com.google.android.packageinstaller",  // 软件包安装程序
    "com.baidu.input",                      //百度输入法
    "com.baidu.input_huawei",               //百度输入法华为版
    "com.baidu.input_mi",                   //百度输入法小米版
    "com.baidu.input_oppo",                 //百度输入法OPPO版
    "com.baidu.input_vivo",                 //百度输入法VIVO版
    "com.baidu.input_yijia",                //百度输入法一加版
    "com.sohu.inputmethod.sogou",           //搜狗输入法
    "com.sohu.inputmethod.sogou.xiaomi",    //搜狗输入法小米版
    "com.sohu.inputmethod.sogou.meizu",     //搜狗输入法魅族版
    "com.sohu.inputmethod.sogou.nubia",     //搜狗输入法nubia版
    "com.sohu.inputmethod.sogou.chuizi",    //搜狗输入法chuizi版
    "com.sohu.inputmethod.sogou.moto",      //搜狗输入法moto版
    "com.sohu.inputmethod.sogou.zte",       //搜狗输入法中兴版
    "com.sohu.inputmethod.sogou.samsung",   //搜狗输入法samsung版
    "com.sohu.input_yijia",                 //搜狗输入法一加版
    "com.iflytek.inputmethod",              //讯飞输入法
    "com.iflytek.inputmethod.miui",         //讯飞输入法小米版
    "com.iflytek.inputmethod.googleplay",   //讯飞输入法googleplay版
    "com.iflytek.inputmethod.smartisan",    //讯飞输入法smartisan版
    "com.iflytek.inputmethod.oppo",         //讯飞输入法oppo版
    "com.iflytek.inputmethod.oem",          //讯飞输入法oem版
    "com.iflytek.inputmethod.custom",       //讯飞输入法custom版
    "com.iflytek.inputmethod.blackshark",   //讯飞输入法blackshark版
    "com.iflytek.inputmethod.zte",          //讯飞输入法zte版
    "com.tencent.qqpinyin",                 // QQ拼音输入法
    "com.google.android.inputmethod.latin", //谷歌Gboard输入法
    "com.touchtype.swiftkey",               //微软swiftkey输入法
    "com.touchtype.swiftkey.beta",          //微软swiftkeyBeta输入法
    "im.weshine.keyboard",                  // KK键盘输入法
    "com.komoxo.octopusime",                //章鱼输入法
    "com.qujianpan.duoduo",                 //见萌输入法
    "com.lxlm.lhl.softkeyboard",            //流行输入法
    "com.jinkey.unfoldedime",               //不折叠输入法
    "com.iflytek.inputmethods.DungkarIME",  //东噶藏文输入法
    "com.oyun.qingcheng",                   //奥云蒙古文输入法
    "com.ziipin.softkeyboard",              // Badam维语输入法
    "com.kongzue.secretinput",              // 密码键盘
    "com.google.android.ext.services",
    "com.google.android.ext.shared",
    "com.google.android.gms",                              // GMS 服务
    "com.google.android.gsf",                              // Google 服务框架
    "com.google.android.systemui.gxoverlay",               // SystemUIGX
    "com.google.android.tag",                              // Tags
    "com.google.android.documentsui",                      // 文件
    "com.google.android.ext.shared",                       // Android Shared Library
    "com.google.android.captiveportallogin",               // 强制门户登录
    "com.google.android.printservice.recommendation",      // Print Service Recommendation Service
    "com.google.android.gms.supervision",                  // Family Link 家长控制
    "com.google.android.as.oss",                           // Private Compute Services
    "com.google.android.configupdater",                    // ConfigUpdater
    "com.google.android.apps.restore",                     // 数据恢复工具
    "com.google.android.onetimeinitializer",               // Google One Time Init
    "com.google.android.odad",                             // Google Play 保护机制服务
    "com.google.android.settings.intelligence",            // 设置小助手
    "com.google.android.partnersetup",                     // Google Partner Setup
    "com.google.android.projection.gearhead",              // Android Auto
    "com.google.android.apps.wellbeing",                   // 数字健康
    "com.google.android.as",                               // Android System Intelligence
    "com.google.android.dialer",                           // 电话
    "com.google.android.apps.messaging",                   // 信息
    "com.google.android.googlequicksearchbox",             // Google
    "com.google.android.webview",                          // Android System WebView
    "com.google.android.tts",                              // Google 语音服务
    "com.google.android.deskclock",                        // 时钟
    "com.google.android.markup",                           // Markup
    "com.google.android.calendar",                         // 日历
    "com.google.android.soundpicker",                      // 音效
    "com.google.android.apps.wallpaper.nexus",             // Google Wallpaper Images
    "com.google.android.modulemetadata",                   // Main components
    "com.google.android.contacts",                         // 通讯录
    "com.google.android.apps.customization.pixel",         // Pixel Themes
    "com.google.android.apps.photos",                      // 相册
    "com.google.android.feedback",                         // 应用商店反馈代理程序
    "com.google.android.apps.wallpaper",                   // 壁纸与个性化
    "com.google.android.ext.services",                     // Android Services Library
    "com.google.android.providers.media.module",           // 媒体
    "com.google.android.wifi.resources",                   // 系统 WLAN 资源
    "com.google.android.hotspot2.osulogin",                // OsuLogin
    "com.google.android.safetycenter.resources",           // Google 安全中心资源
    "com.google.android.permissioncontroller",             // 权限控制器
    "com.google.android.ondevicepersonalization.services", //
    "com.google.android.adservices.api",                   // Android 系统
    "com.google.android.devicelockcontroller",             // DeviceLockController
    "com.google.android.connectivity.resources",           // 系统网络连接资源
    "com.google.android.healthconnect.controller",         // Health Connect
    "com.google.android.cellbroadcastreceiver",            // 无线紧急警报
    "com.google.android.uwb.resources",                    // System UWB Resources
    "com.google.android.rkpdapp",                          // RemoteProvisioner
    "com.android.launcher",
    "com.android.launcher2",
    "com.android.launcher3",
    "com.android.launcher4",
    "com.android.camera",
    "com.android.camera2",
    "com.android.apps.tag",                                          // Tags
    "com.android.bips",                                              // 系统打印服务
    "com.android.bluetoothmidiservice",                              // Bluetooth MIDI Service
    "com.android.cameraextensions",                                  // CameraExtensionsProxy
    "com.android.captiveportallogin",                                // CaptivePortalLogin
    "com.android.carrierdefaultapp",                                 // 运营商默认应用
    "com.android.certinstaller",                                     // 证书安装程序
    "com.android.companiondevicemanager",                            // 配套设备管理器
    "com.android.connectivity.resources",                            // 系统网络连接资源
    "com.android.contacts",                                          // 通讯录与拨号
    "com.android.deskclock",                                         // 时钟
    "com.android.dreams.basic",                                      // 基本互动屏保
    "com.android.egg",                                               // Android S Easter Egg
    "com.android.emergency",                                         // 急救信息
    "com.android.externalstorage",                                   // 外部存储设备
    "com.android.hotspot2.osulogin",                                 // OsuLogin
    "com.android.htmlviewer",                                        // HTML 查看器
    "com.android.incallui",                                          // 电话
    "com.android.internal.display.cutout.emulation.corner",          // 边角刘海屏
    "com.android.internal.display.cutout.emulation.double",          // 双刘海屏
    "com.android.internal.display.cutout.emulation.hole",            // 打孔屏
    "com.android.internal.display.cutout.emulation.tall",            // 长型刘海屏
    "com.android.internal.display.cutout.emulation.waterfall",       // 瀑布刘海屏
    "com.android.internal.systemui.navbar.gestural",                 // Gestural Navigation Bar
    "com.android.internal.systemui.navbar.gestural_extra_wide_back", // Gestural Navigation Bar
    "com.android.internal.systemui.navbar.gestural_narrow_back",     // Gestural Navigation Bar
    "com.android.internal.systemui.navbar.gestural_wide_back",       // Gestural Navigation Bar
    "com.android.internal.systemui.navbar.threebutton",              // 3 Button Navigation Bar
    "com.android.managedprovisioning",                               // 工作设置
    "com.android.mms",                                               // 短信
    "com.android.modulemetadata",                                    // Module Metadata
    "com.android.mtp",                                               // MTP 主机
    "com.android.musicfx",                                           // MusicFX
    "com.android.networkstack.inprocess.overlay", // NetworkStackInProcessResOverlay
    "com.android.networkstack.overlay",           // NetworkStackOverlay
    "com.android.networkstack.tethering.inprocess.overlay", // TetheringResOverlay
    "com.android.networkstack.tethering.overlay", // TetheringResOverlay
    "com.android.packageinstaller",               // 软件包安装程序
    "com.android.pacprocessor",                   // PacProcessor
    "com.android.permissioncontroller",           // 权限控制器
    "com.android.printspooler",                   // 打印处理服务
    "com.android.providers.calendar",             // 日历存储
    "com.android.providers.contacts",             // 联系人存储
    "com.android.providers.downloads.ui",         // 下载管理
    "com.android.providers.media.module",         // 媒体存储设备
    "com.android.proxyhandler",                   // ProxyHandler
    "com.android.server.telecom.overlay.miui",    // 通话管理
    "com.android.settings.intelligence",          // 设置建议
    "com.android.simappdialog",                   // Sim App Dialog
    "com.android.soundrecorder",                  // 录音机
    "com.android.statementservice",               // 意图过滤器验证服务
    "com.android.storagemanager",                 // 存储空间管理器
    "com.android.theme.font.notoserifsource",     // Noto Serif / Source Sans Pro
    "com.android.traceur",                        // 系统跟踪
    "com.android.uwb.resources",                  // System UWB Resources
    "com.android.vpndialogs",                     // VpnDialogs
    "com.android.wallpaper.livepicker",           // Live Wallpaper Picker
    "com.android.wifi.resources",                 // 系统 WLAN 资源
    "com.debug.loggerui",                         // DebugLoggerUI
    "com.fingerprints.sensortesttool",            // Sensor Test Tool
    "com.lbe.security.miui",                      // 权限管理服务
    "com.mediatek.callrecorder",                  // 通话录音机
    "com.mediatek.duraspeed",                     // 快霸
    "com.mediatek.engineermode",                  // EngineerMode
    "com.mediatek.lbs.em2.ui",                    // LocationEM2
    "com.mediatek.location.mtkgeofence",          // Mtk Geofence
    "com.mediatek.mdmconfig",                     // MDMConfig
    "com.mediatek.mdmlsample",                    // MDMLSample
    "com.mediatek.miravision.ui",                 // MiraVision
    "com.mediatek.op01.telecom",                  // OP01Telecom
    "com.mediatek.op09clib.phone.plugin",         // OP09ClibTeleService
    "com.mediatek.op09clib.telecom",              // OP09ClibTelecom
    "com.mediatek.ygps",                          // YGPS
    "com.tencent.soter.soterserver",              // SoterService
    "com.unionpay.tsmservice.mi",                 // 银联可信服务安全组件小米版本
    "android.ext.services",                       // Android Services Library
    "android.ext.shared",                         // Android Shared Library
    "com.android.adservices.api",                 // Android AdServices
    "com.android.bookmarkprovider",               // Bookmark Provider
    "com.android.cellbroadcastreceiver.module",   // 无线紧急警报
    "com.android.dialer",                         // 电话
    "com.android.dreams.phototable",              // 照片屏幕保护程序
    "com.android.inputmethod.latin",              // Android 键盘 (AOSP)
    "com.android.intentresolver",                 // IntentResolver
    "com.android.internal.display.cutout.emulation.noCutout", // 隐藏
    "com.android.internal.systemui.navbar.twobutton", // 2 Button Navigation Bar
    "com.android.messaging",                      // 短信
    "com.android.onetimeinitializer",             // One Time Init
    "com.android.printservice.recommendation",    // Print Service Recommendation Service
    "com.android.safetycenter.resources",         // 安全中心资源
    "com.android.soundpicker",                    // 声音
    "com.android.systemui",                       // 系统界面
    "com.android.wallpaper",                      // 壁纸和样式
    "com.qualcomm.qti.cne",                       // CneApp
    "com.qualcomm.qti.poweroffalarm",             // 关机闹钟
    "com.qualcomm.wfd.service",                   // Wfd Service
    "org.lineageos.aperture",                     // 相机
    "org.lineageos.audiofx",                      // AudioFX
    "org.lineageos.backgrounds",                  // 壁纸
    "org.lineageos.customization",                // Lineage Themes
    "org.lineageos.eleven",                       // 音乐
    "org.lineageos.etar",                         // 日历
    "org.lineageos.jelly",                        // 浏览器
    "org.lineageos.overlay.customization.blacktheme", // Black theme
    "org.lineageos.overlay.font.lato",            // Lato
    "org.lineageos.overlay.font.rubik",           // Rubik
    "org.lineageos.profiles",                     // 情景模式信任提供器
    "org.lineageos.recorder",                     // 录音机
    "org.lineageos.updater",                      // 系统更新
    "org.protonaosp.deviceconfig",                // Simple Device Configuration
    "android.aosp.overlay",
    "android.miui.home.launcher.res",
    "android.miui.overlay",
    "com.android.carrierconfig",
    "com.android.carrierconfig.overlay.miui",
    "com.android.incallui.overlay",
    "com.android.managedprovisioning.overlay",
    "com.android.ondevicepersonalization.services",
    "com.android.overlay.cngmstelecomm",
    "com.android.overlay.gmscontactprovider",
    "com.android.overlay.gmssettingprovider",
    "com.android.overlay.gmssettings",
    "com.android.overlay.gmstelecomm",
    "com.android.overlay.gmstelephony",
    "com.android.overlay.systemui",
    "com.android.phone.overlay.miui",
    "com.android.providers.settings.overlay",
    "com.android.sdksandbox",
    "com.android.settings.overlay.miui",
    "com.android.stk.overlay.miui",
    "com.android.systemui.gesture.line.overlay",
    "com.android.systemui.navigation.bar.overlay",
    "com.android.systemui.overlay.miui",
    "com.android.wallpapercropper",
    "com.android.wallpaperpicker",
    "com.android.wifi.dialog",
    "com.android.wifi.resources.overlay",
    "com.android.wifi.resources.xiaomi",
    "com.android.wifi.system.mainline.resources.overlay",
    "com.android.wifi.system.resources.overlay",
    "com.google.android.cellbroadcastreceiver.overlay.miui",
    "com.google.android.cellbroadcastservice.overlay.miui",
    "com.google.android.overlay.gmsconfig",
    "com.google.android.overlay.modules.ext.services",
    "com.google.android.trichromelibrary_511209734",
    "com.google.android.trichromelibrary_541411734",
    "com.mediatek.FrameworkResOverlayExt",
    "com.mediatek.SettingsProviderResOverlay",
    "com.mediatek.batterywarning",
    "com.mediatek.cellbroadcastuiresoverlay",
    "com.mediatek.frameworkresoverlay",
    "com.mediatek.gbaservice",
    "com.mediatek.voiceunlock",
    "com.miui.core.internal.services",
    "com.miui.face.overlay.miui",
    "com.miui.miwallpaper.overlay.customize",
    "com.miui.miwallpaper.wallpaperoverlay.config.overlay",
    "com.miui.rom",
    "com.miui.settings.rro.device.config.overlay",
    "com.miui.settings.rro.device.hide.statusbar.overlay",
    "com.miui.settings.rro.device.type.overlay",
    "com.miui.system.overlay",
    "com.miui.systemui.carriers.overlay",
    "com.miui.systemui.devices.overlay",
    "com.miui.systemui.overlay.devices.android",
    "com.miui.translation.kingsoft",
    "com.miui.translation.xmcloud",
    "com.miui.translationservice",
    "com.miui.voiceassistoverlay",
    "com.miui.wallpaper.overlay.customize",
    "com.xiaomi.bluetooth.rro.device.config.overlay",
    "android.auto_generated_rro_product__",
    "android.auto_generated_rro_vendor__",
    "com.android.backupconfirm",
    "com.android.carrierconfig.auto_generated_rro_vendor__",
    "com.android.cts.ctsshim",
    "com.android.cts.priv.ctsshim",
    "com.android.documentsui.auto_generated_rro_product__",
    "com.android.emergency.auto_generated_rro_product__",
    "com.android.imsserviceentitlement",
    "com.android.imsserviceentitlement.auto_generated_rro_product__",
    "com.android.inputmethod.latin.auto_generated_rro_product__",
    "com.android.launcher3.overlay",
    "com.android.managedprovisioning.auto_generated_rro_product__",
    "com.android.nearby.halfsheet",
    "com.android.phone.auto_generated_rro_vendor__",
    "com.android.providers.settings.auto_generated_rro_product__",
    "com.android.providers.settings.auto_generated_rro_vendor__",
    "com.android.settings.auto_generated_rro_product__",
    "com.android.sharedstoragebackup",
    "com.android.smspush",
    "com.android.storagemanager.auto_generated_rro_product__",
    "com.android.systemui.auto_generated_rro_product__",
    "com.android.systemui.auto_generated_rro_vendor__",
    "com.android.systemui.plugin.globalactions.wallet",
    "com.android.wallpaper.auto_generated_rro_product__",
    "com.android.wifi.resources.oneplus_sdm845",
    "com.qualcomm.timeservice",
    "lineageos.platform.auto_generated_rro_product__",
    "lineageos.platform.auto_generated_rro_vendor__",
    "org.codeaurora.ims",
    "org.lineageos.aperture.auto_generated_rro_vendor__",
    "org.lineageos.lineageparts.auto_generated_rro_product__",
    "org.lineageos.lineagesettings.auto_generated_rro_product__",
    "org.lineageos.lineagesettings.auto_generated_rro_vendor__",
    "org.lineageos.overlay.customization.navbar.nohint",
    "org.lineageos.settings.device.auto_generated_rro_product__",
    "org.lineageos.settings.doze.auto_generated_rro_product__",
    "org.lineageos.settings.doze.auto_generated_rro_vendor__",
    "org.lineageos.setupwizard.auto_generated_rro_product__",
    "org.lineageos.updater.auto_generated_rro_product__",
    "org.protonaosp.deviceconfig.auto_generated_rro_product__",
    "com.mi.health",                    // 小米运动健康
    "com.tencent.mm.wxa.sce",           // 微信小程序   三星OneUI专用
    "com.onlyone.onlyonestarter",       // 三星系应用
    "com.samsung.accessory.neobeanmgr", // Galaxy Buds Live Manager
    "com.samsung.app.newtrim",          // 编辑器精简版
    "com.diotek.sec.lookup.dictionary", // 字典
];

pub struct App {
    pub home_uid: usize,
    has_home: bool,
    AllPackages: HashMap<String, usize>,
    pub BackGroundPackages: HashMap<String, usize>,
    pub VisiblePackage: HashMap<String, usize>,
    whitelist: HashSet<usize>,
}

impl App {
    pub fn new() -> Result<Self> {
        let output = Command::new("/system/bin/cmd")
            .args(["package", "list", "packages", "-U"])
            .output()
            .context("无法执行 cmd package list packages -U")?;
        let output = String::from_utf8_lossy(&output.stdout);
        let mut packages = HashMap::new();
        let mut whitelist = HashSet::new();

        for line in output.lines() {
            if line.starts_with("package:") {
                let Some((pkg_part, uid_part)) = line.split_once(" uid:") else {
                    continue;
                };
                let package = pkg_part.trim_start_matches("package:").trim();

                let uid_str = uid_part.split_whitespace().next().unwrap_or("");
                let uid: usize = uid_str
                    .parse()
                    .context(format!("无效的 UID: {}", uid_str))?;

                if uid < 1000 {
                    continue;
                }

                if WHITE_LIST.contains(&package) {
                    whitelist.insert(uid);
                }

                packages.insert(package.to_string(), uid);
            }
        }
        /*#[cfg(debug_assertions)]
        {
            log::info!("{:?}", packages);
            log::info!("{:?}", whitelist);
        }*/
        Ok(Self {
            home_uid: 0,
            AllPackages: packages,
            VisiblePackage: HashMap::new(),
            BackGroundPackages: HashMap::new(),
            has_home: false,
            whitelist,
        })
    }

    pub fn GetPids(uid: usize) -> Result<Vec<usize>> {
        let all_procs = procfs::process::all_processes().context("Failed to list processes")?;

        let mut pids = Vec::new();
        for proc in all_procs {
            let proc = proc.context("Failed to get process info")?;
            if let Ok(status) = proc.status() {
                if status.ruid == uid {
                    pids.push(proc.pid() as usize);
                }
            }
        }
        Ok(pids)
    }

    pub fn ReflashPackages(&mut self) {
        let output = Command::new("/system/bin/cmd")
            .args(["activity", "stack", "list"])
            .output()
            .expect("无法执行cmd activity stack list");
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines = output_str.lines();
        let mut last_line = "";
        for line in lines {
            if !self.has_home && line.contains("mActivityType=home") {
                self.has_home = true;
                last_line = line;
                continue;
            }
            if line.starts_with("  taskId=") {
                if let Some(caps) = APP_REGEX.captures(line) {
                    let package = caps.get(1).unwrap().as_str();
                    if self.AllPackages.contains_key(package) {
                        let uid = *self.AllPackages.get(package).unwrap();
                        if last_line.contains("mActivityType=home") {
                            self.home_uid = uid;
                        }
                    }
                }
            }
            if line.starts_with("  taskId=") {
                if let Some(caps) = APP_REGEX.captures(line) {
                    let package = caps.get(1).unwrap().as_str();
                    if self.AllPackages.contains_key(package) {
                        let uid = *self.AllPackages.get(package).unwrap();
                        if !self.is_whitelist(uid) {
                            if line.contains("visible=true") {
                                self.VisiblePackage.insert(package.to_string(), uid);
                            } else if line.contains("visible=false") {
                                self.BackGroundPackages.insert(package.to_string(), uid);
                            }
                        }
                    }
                }
            }
        }
    }

    fn is_whitelist(&self, uid: usize) -> bool {
        self.whitelist.contains(&uid)
    }
}
