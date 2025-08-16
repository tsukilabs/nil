// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

package tsukilabs.nil

import android.content.res.Configuration
import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    this.hideStatusBar()
  }

  override fun onResume() {
    super.onResume()
    this.hideStatusBar()
  }

  override fun onConfigurationChanged(newConfig: Configuration) {
    super.onConfigurationChanged(newConfig)
    this.hideStatusBar()
  }

  override fun onWindowFocusChanged(hasFocus: Boolean) {
    super.onWindowFocusChanged(hasFocus)
    if (hasFocus) this.hideStatusBar()
  }

  private fun hideStatusBar() {
    WindowCompat.getInsetsController(window, window.decorView).apply {
      this.setSystemBarsBehavior(BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
      this.hide(WindowInsetsCompat.Type.statusBars())
    }
  }
}
