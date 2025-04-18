package com.ruu.freeze;

import android.view.LayoutInflater;
import android.view.ViewGroup;
import androidx.annotation.NonNull;
import androidx.recyclerview.widget.RecyclerView;
import com.bumptech.glide.Glide;
import com.ruu.freeze.databinding.ItemAppBinding;
import java.util.List;

public class AppListAdapter extends RecyclerView.Adapter<AppListAdapter.ViewHolder> {
  private final List<AppInfo> apps;

  public AppListAdapter(List<AppInfo> apps) {
    this.apps = apps;
  }

  @NonNull
  @Override
  public ViewHolder onCreateViewHolder(@NonNull ViewGroup parent, int viewType) {
    LayoutInflater inflater = LayoutInflater.from(parent.getContext());
    ItemAppBinding binding = ItemAppBinding.inflate(inflater, parent, false);
    return new ViewHolder(binding);
  }

  @Override
  public void onBindViewHolder(@NonNull ViewHolder holder, int position) {
    AppInfo app = apps.get(position);
    holder.binding.tvAppName.setText(app.getName());

    Glide.with(holder.itemView.getContext())
        .load(app.getIcon())
        .circleCrop()
        .into(holder.binding.ivAppIcon);
  }

  @Override
  public int getItemCount() {
    return apps.size();
  }

  static class ViewHolder extends RecyclerView.ViewHolder {
    ItemAppBinding binding;

    public ViewHolder(ItemAppBinding binding) {
      super(binding.getRoot());
      this.binding = binding;
    }
  }
}
