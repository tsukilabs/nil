package tsukilabs.nil

import android.os.Bundle
import android.view.View
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)

    window.decorView.systemUiVisibility = View.SYSTEM_UI_FLAG_FULLSCREEN
  }

  override fun onResume() {
    super.onResume()

    window.decorView.systemUiVisibility = View.SYSTEM_UI_FLAG_FULLSCREEN
  }

  override fun onWindowFocusChanged(hasFocus: Boolean) {
    super.onWindowFocusChanged(hasFocus)

    if (hasFocus) {
      window.decorView.systemUiVisibility = View.SYSTEM_UI_FLAG_FULLSCREEN
    }
  }
}
