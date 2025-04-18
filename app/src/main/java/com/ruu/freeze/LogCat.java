package com.ruu.freeze;

import android.os.Bundle;
import android.util.Log;
import androidx.appcompat.app.AppCompatActivity;
import com.ruu.freeze.databinding.LayoutLogCatBinding;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.InetSocketAddress;
import java.net.Socket;

public class LogCat extends AppCompatActivity {
  private LayoutLogCatBinding binding;
  private volatile boolean isRunning = true;
  private Socket socket;
  private BufferedReader reader;

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    binding = LayoutLogCatBinding.inflate(getLayoutInflater());
    setContentView(binding.getRoot());

    new Thread(
            () -> {
              while (isRunning) {
                try {
                  socket = new Socket();
                  socket.connect(new InetSocketAddress("0.0.0.0", 25560), 5000);

                  Log.d("Socket", "Connected");
                  reader = new BufferedReader(new InputStreamReader(socket.getInputStream()));

                  String line;
                  while (isRunning && (line = reader.readLine()) != null) {
                    updateLogView(line);
                  }
                } catch (Exception e) {
                  Log.e("Socket", "Error: " + e.getMessage());
                  updateLogView("无Socket连接，请检查模块状态");
                  try {
                    Thread.sleep(2000);
                  } catch (InterruptedException ex) {
                    break;
                  }
                } finally {
                  closeResources();
                }
              }
            })
        .start();
  }

  private void updateLogView(final String msg) {
    runOnUiThread(
        () -> {
          binding.logView.append(msg + "\n");
          binding.logScrollView.post(
              () -> {
                binding.logScrollView.smoothScrollTo(0, binding.forBottom.getBottom());
              });
        });
  }

  @Override
  protected void onDestroy() {
    super.onDestroy();
    isRunning = false;
    closeResources();
    binding = null;
  }

  @Override
  public void onResume() {
    super.onResume();
    isRunning = false;
    binding.LogSwipRefresh.setRefreshing(false);
    closeResources();
  }

  private void closeResources() {
    try {
      if (reader != null) reader.close();
      if (socket != null) socket.close();
    } catch (IOException e) {
      Log.e("SocketClose", e.getMessage());
    }
  }
}
