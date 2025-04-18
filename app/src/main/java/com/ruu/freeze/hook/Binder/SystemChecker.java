package com.ruu.freeze.hook.Binder;

import de.robv.android.xposed.XposedHelpers;

public class SystemChecker {
    public static boolean isXiaomi(ClassLoader classLoader) {
        return XposedHelpers.findClassIfExists("com.miui.server.greeze.GreezeManagerService", classLoader) != null;
    }
}