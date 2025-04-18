package com.ruu.freeze;

import android.content.Intent;
import android.os.Bundle;
import android.util.Log;
import androidx.appcompat.app.AppCompatActivity;
import androidx.swiperefreshlayout.widget.SwipeRefreshLayout;
import com.gyf.immersionbar.ImmersionBar;
import com.ruu.freeze.databinding.ActivityMainBinding;

public class MainActivity extends AppCompatActivity {
  private ActivityMainBinding binding;

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);

    binding = ActivityMainBinding.inflate(getLayoutInflater());

    ImmersionBar.with(this).transparentBar().init();
    setContentView(binding.getRoot());

    binding.LogCat.setOnClickListener(
        v -> {
          startActivity(new Intent(MainActivity.this, LogCat.class));
        });
    binding.Settings.setOnClickListener(
        v -> {
          startActivity(new Intent(MainActivity.this, Settings.class));
        });
    
    binding.AppList.setOnClickListener(
        v -> {
          startActivity(new Intent(MainActivity.this, AppList.class));
        });
    binding.container.setOnRefreshListener(
        () -> {
          Refresh();
        });
  }

  @Override
  protected void onDestroy() {
    super.onDestroy();
    this.binding = null;
  }

  @Override
  public void onResume() {
    super.onResume();
        binding.container.setBackground(Data.getBackgroundDrawable(this));
    Refresh();
  }

  private void Refresh() {
    boolean xposedState = isXposedActive();
    if (xposedState) {
      binding.statusText.setText("运行中");
    } else {
      binding.statusText.setText("未运行");
    }
    binding.container.setRefreshing(false);
  }

  public boolean isXposedActive() {
    Log.e("Freeze Init", "isXposedActive: Hook Fail");
    return false;
  }
}
