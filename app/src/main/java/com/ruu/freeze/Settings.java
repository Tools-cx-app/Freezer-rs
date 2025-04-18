package com.ruu.freeze;

import android.content.Context;
import android.content.Intent;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.graphics.drawable.BitmapDrawable;
import android.os.Bundle;
import androidx.activity.result.ActivityResultLauncher;
import androidx.appcompat.app.AppCompatActivity;
import androidx.activity.result.contract.ActivityResultContracts;
import com.ruu.freeze.databinding.LayoutSettingsBinding;

public class Settings extends AppCompatActivity {
  private LayoutSettingsBinding binding;
  ActivityResultLauncher<Intent> pickPicture;

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    binding = LayoutSettingsBinding.inflate(getLayoutInflater());
    setContentView(binding.getRoot());

    pickPicture =
        registerForActivityResult(
            new ActivityResultContracts.StartActivityForResult(),
            result -> {
              if (result.getResultCode() != RESULT_OK
                  || result.getData() == null
                  || result.getData().getData() == null) return;

              try {
                String imagePath = Utils.getFileAbsolutePath(this, result.getData().getData());
                var bg = BitmapFactory.decodeFile(imagePath);
                if (bg == null || bg.getHeight() == 0 || bg.getWidth() == 0) return;

                // 居中截取 宽:高 = 1:2
                if (bg.getHeight() > 2 * bg.getWidth())
                  bg =
                      Bitmap.createBitmap(
                          bg,
                          0,
                          bg.getHeight() / 2 - bg.getWidth(),
                          bg.getWidth(),
                          bg.getWidth() * 2);
                else if (bg.getHeight() < 2 * bg.getWidth())
                  bg =
                      Bitmap.createBitmap(
                          bg,
                          bg.getWidth() / 2 - bg.getHeight() / 4,
                          0,
                          bg.getHeight() / 2,
                          bg.getHeight());

                // 限制分辨率
                if (bg.getWidth() > 1080) bg = Utils.resize(bg, 1080f / bg.getWidth());

                bg.compress(
                    Bitmap.CompressFormat.JPEG,
                    90,
                    openFileOutput(Data.bgFileName, Context.MODE_PRIVATE));

                Data.bg = new BitmapDrawable(getResources(), bg);
                Data.bg.setAlpha(56);
              } catch (Exception ignore) {
              }
            });

    binding.setBackground.setOnClickListener(
        v -> {
          Intent intent = new Intent("android.intent.action.GET_CONTENT");
          intent.setType("image/*");
          pickPicture.launch(intent);
        });
  }

  @Override
  public void onResume() {
    super.onResume();
    binding.container.setBackground(Data.getBackgroundDrawable(this));
  }
}
