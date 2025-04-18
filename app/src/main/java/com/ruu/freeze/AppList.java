package com.ruu.freeze;

import android.content.pm.ApplicationInfo;
import android.content.pm.PackageManager;
import android.graphics.drawable.Drawable;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.widget.ArrayAdapter;
import androidx.appcompat.app.AppCompatActivity;
import androidx.recyclerview.widget.LinearLayoutManager;
import com.ruu.freeze.databinding.LayoutAppListBinding;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class AppList extends AppCompatActivity {
  private LayoutAppListBinding binding;
  private final ExecutorService executor = Executors.newSingleThreadExecutor();
  private final Handler mainHandler = new Handler(Looper.getMainLooper());

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    binding = LayoutAppListBinding.inflate(getLayoutInflater());
    setContentView(binding.getRoot());

    binding.container.setOnRefreshListener(
        () -> {
          loadAppsAsync();
        });
  }

  private void loadAppsAsync() {
    executor.execute(
        () -> {
          List<AppInfo> apps = Utils.getInstalledApps(AppList.this);

          mainHandler.post(
              () -> {
                AppListAdapter adapter = new AppListAdapter(apps);
                binding.recyclerviewApp.setLayoutManager(new LinearLayoutManager(AppList.this));
                binding.recyclerviewApp.setAdapter(adapter);
              });
        });
    binding.container.setRefreshing(false);
  }

  @Override
  protected void onDestroy() {
    super.onDestroy();
    executor.shutdown();
    binding = null;
  }

  @Override
  public void onResume() {
    super.onResume();
    binding.container.setBackground(Data.getBackgroundDrawable(this));
    loadAppsAsync();
    // binding = null;
  }
}
