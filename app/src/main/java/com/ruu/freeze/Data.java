package com.ruu.freeze;
import android.annotation.SuppressLint;
import android.app.ActivityManager;
import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.drawable.Drawable;

public class Data {
    public static final String bgFileName = "bg.jpg";
    public static Drawable bg;

    @SuppressLint("UseCompatLoadingForDrawables")
    public static Drawable getBackgroundDrawable(Context context){
        if(bg == null) {
            try {
                bg = Drawable.createFromPath(
                        context.getFilesDir().getPath() + "/" + bgFileName);

                if (bg == null)
                    throw new Exception();
                bg.setAlpha(56);
            } catch (Exception ignored) {
                bg = context.getDrawable(R.drawable.background);
                bg.setAlpha(50);
            }
        }
        return bg;
    }
}
