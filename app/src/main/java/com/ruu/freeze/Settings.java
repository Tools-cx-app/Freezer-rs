package com.ruu.freeze;

import android.os.Bundle;
import androidx.appcompat.app.AppCompatActivity;
import com.ruu.freeze.databinding.LayoutSettingsBinding;

public class Settings extends AppCompatActivity {
  private LayoutSettingsBinding binding;

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    binding = LayoutSettingsBinding.inflate(getLayoutInflater());
    setContentView(binding.getRoot());
  }

  @Override
  public void onResume() {
    super.onResume();
    binding.container.setBackground(Data.getBackgroundDrawable(this));
  }
}
