package com.ruu.freeze.hook.android;

import android.os.Build;

import de.robv.android.xposed.XC_MethodHook;
import de.robv.android.xposed.XposedHelpers;
import com.ruu.freeze.base.AbstractMethodHook;
import com.ruu.freeze.base.MethodHook;
import com.ruu.freeze.hook.Config;
import com.ruu.freeze.hook.XpUtils;
import com.ruu.freeze.hook.Enum;
// 广播滤波器跳过Hook.
public class BroadHook extends MethodHook {
    private final Config config;

    public BroadHook(ClassLoader classLoader, Config config) {
        super(classLoader);
        this.config = config;
    }

    @Override
    public String getTargetClass() {
        return "com.android.server.am.BroadcastSkipPolicy";
    }

    @Override
    public String getTargetMethod() {
        return "shouldSkipMessage";
    }

    @Override
    public Object[] getTargetParam() {
        if (XposedHelpers.findClassIfExists("com.android.server.am.IVivoBroadcastQueueModern", classLoader) != null)
            return new Object[]{ "com.android.server.am.BroadcastRecord", "com.android.server.am.BroadcastFilter", boolean.class, int.class, "com.android.server.am.IVivoBroadcastQueueModern" };
        return new Object[]{ "com.android.server.am.BroadcastRecord", "com.android.server.am.BroadcastFilter" };
    }

    @Override
    public XC_MethodHook getTargetHook() {
        return new AbstractMethodHook() {
            @Override
            protected void afterMethod(XC_MethodHook.MethodHookParam param) {
                if (param.getResult() != null)
                    return;

                final int uid = config.getBroadcastFilterOwningUid(param.args[1]);// BroadcastFilter

                // 不在管理范围，或顶层前台 则不清理广播
                if (!config.managedApp.contains(uid) || config.foregroundUid.contains(uid))
                    return;

                param.setResult("Skipping deliver [Frozen]: frozen process");
                if (XpUtils.DEBUG_BROADCAST_STATIC) {
                    // BroadcastRecord https://cs.android.com/android/platform/superproject/+/master:frameworks/base/services/core/java/com/android/server/am/BroadcastRecord.java
                    final int callerUid = config.getBroadcastRecordCallingUid(param.args[0]); // broadcastRecord
                    XpUtils.log("Frozen[BroadCastHook]:", "拦截动态广播: " +
                            config.pkgIndex.getOrDefault(callerUid, String.valueOf(callerUid)) +
                            " 发往 " +
                            config.pkgIndex.getOrDefault(uid, String.valueOf(uid))
                    );
                }
            }
            protected void beforeHookedMethod(MethodHookParam param) {
                final int uid = config.getProcessRecordUid(param.args[1]); // ProcessRecord
                if (!config.managedApp.contains(uid) || config.foregroundUid.contains(uid)) return;
                param.setResult(null);
                if (XpUtils.DEBUG_BROADCAST_STATIC) {
                    final int callerUid = config.getBroadcastRecordCallingUid(param.args[0]); // BroadcastRecord
                    XpUtils.log("Frozen[BroadCastHook]:", "拦截静态广播: " +
                            config.pkgIndex.getOrDefault(callerUid, String.valueOf(callerUid)) +
                            " 发往 " +
                            config.pkgIndex.getOrDefault(uid, String.valueOf(uid))
                    );
                }
                XposedHelpers.findAndHookMethod(  Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE ?
                                Enum.Class.BroadcastQueueImpl : Enum.Class.BroadcastQueue,
                        classLoader, Enum.Method.skipReceiverLocked,
                        Enum.Class.BroadcastRecord);
            }
        };
    }
    @Override
    public int getMinVersion() {
        return Build.VERSION_CODES.UPSIDE_DOWN_CAKE;
    }
    @Override
    public String successLog() {
        return "修改跳过广播";
    }
}
