plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.blinc.example"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.blinc.example"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        ndk {
            abiFilters += listOf("arm64-v8a")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
}

tasks.register<Copy>("copyRustLibs") {
    val rustTargetDir = file("../../../../target")
    val jniLibsDir = file("src/main/jniLibs")

    from("$rustTargetDir/aarch64-linux-android/debug") {
        include("libexample.so")
        into("arm64-v8a")
    }

    into(jniLibsDir)
}

tasks.named("preBuild") {
    dependsOn("copyRustLibs")
}
