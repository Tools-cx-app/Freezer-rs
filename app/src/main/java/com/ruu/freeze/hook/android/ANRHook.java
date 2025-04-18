package com.ruu.freeze.hook.android;

import android.content.pm.ApplicationInfo;
import android.os.Build;

import de.robv.android.xposed.XC_MethodHook;
import de.robv.android.xposed.XposedHelpers;
import com.ruu.freeze.base.AbstractMethodHook;
import com.ruu.freeze.base.MethodHook;
import com.ruu.freeze.hook.Config;
import com.ruu.freeze.hook.Enum;
import com.ruu.freeze.hook.XpUtils;

// ANR相关Hook.
public class ANRHook extends MethodHook {
    private final Config config;

    public ANRHook(ClassLoader classLoader, Config config) {
        super(classLoader);
        this.config = config;
    }

    @Override
    public String getTargetClass() {
        if (Build.VERSION.SDK_INT > Build.VERSION_CODES.Q) {
            return "com.android.server.am.AnrHelper$AnrRecord";
        } else if (Build.VERSION.SDK_INT == Build.VERSION_CODES.Q) {
            return  Enum.Class.ProcessRecord;
        } else {
            return "com.android.server.am.AppErrors";
        }
    }

    @Override
    public String getTargetMethod() {
        return "appNotResponding";
    }

    @Override
    public Object[] getTargetParam() {
        if (Build.VERSION.SDK_INT > Build.VERSION_CODES.Q) {
            return new Object[]{ boolean.class };
        } else if (Build.VERSION.SDK_INT == Build.VERSION_CODES.Q) {
            return new Object[]{
                    String.class, ApplicationInfo.class, String.class,
                    Enum.Class.WindowProcessController, boolean.class, String.class
            };
        } else {
            return new Object[]{
                    "com.android.server.am.ProcessRecord", "com.android.server.am.ActivityRecord",
                    "com.android.server.am.ActivityRecord", boolean.class, String.class
            };
        }
    }

    @Override
    public XC_MethodHook getTargetHook() {
        return new AbstractMethodHook() {
            @Override
            protected void beforeMethod(MethodHookParam param) {
                // ANR进程
                Object app;
                if (Build.VERSION.SDK_INT > Build.VERSION_CODES.Q)
                    app = XposedHelpers.getObjectField(param.thisObject, "mApp");
                else if (Build.VERSION.SDK_INT == Build.VERSION_CODES.Q)
                    app = param.thisObject;
                else
                    app = param.args[0];
                if (app == null)
                    return;
                final int uid = config.getProcessRecordUid(app);// processRecord
                if (!config.managedApp.contains(uid))
                    return;
                param.setResult(null);
                if (XpUtils.DEBUG_ANR)
                    XpUtils.log("Frozen[AnrHook]:", "跳过 ANR:" + XpUtils.getString(app, Enum.Field.processName));
            }
        };
    }

    @Override
    public int getMinVersion() {
        return Build.VERSION_CODES.P;
    }

    @Override
    public String successLog() {
        return "拦截应用持续ANR";
    }
}
