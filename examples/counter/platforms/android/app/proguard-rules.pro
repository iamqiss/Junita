# Junita ProGuard rules
# Keep Junita runtime classes
-keep class junita.** { *; }

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}
