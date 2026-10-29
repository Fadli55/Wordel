package ai.storyteller.wordcraft

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.util.Log
import com.google.androidgamesdk.GameActivity

class MainActivity : GameActivity() {

    companion object {
        private const val TAG = "WordCraftActivity"

        init {
            try {
                System.loadLibrary("wordcraft_android")
                Log.i(TAG, "Successfully loaded native library wordcraft_android")
            } catch (e: UnsatisfiedLinkError) {
                Log.e(TAG, "Failed to load native library wordcraft_android: ${e.message}")
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        handleIntent(intent)
    }

    override fun onNewIntent(intent: Intent?) {
        super.onNewIntent(intent)
        setIntent(intent)
        intent?.let { handleIntent(it) }
    }

    private fun handleIntent(intent: Intent) {
        val uri: Uri? = intent.data
        if (uri != null) {
            Log.i(TAG, "Opened WordCraft with document URI: $uri")
            // Document URI can be read via ContentResolver and forwarded to Rust inbox
        }
    }
}
