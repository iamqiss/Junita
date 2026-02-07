package com.junita.example

import android.app.NativeActivity
import android.os.Bundle

class MainActivity : NativeActivity() {
    companion object {
        init {
            System.loadLibrary("example")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
    }
}
