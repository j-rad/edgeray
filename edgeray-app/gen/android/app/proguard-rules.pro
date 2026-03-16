# EdgeRay Android ProGuard Rules
# Prevents obfuscation of JNI interfaces and Rust FFI entry points

# Keep all EdgeRay classes
-keep class com.edgeray.** { *; }

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep JNI interfaces for Rust FFI
-keep class com.edgeray.TunnelService {
    public <methods>;
    public <fields>;
}

-keep class com.edgeray.TunnelService$* {
    *;
}

# Keep VpnService implementation
-keep class com.edgeray.EdgeRayVpnService {
    public <methods>;
}

# Keep callback interfaces
-keep interface com.edgeray.callbacks.** { *; }

# Keep data classes used in JNI
-keep class com.edgeray.models.** {
    public <fields>;
    public <methods>;
}

# Keep enums
-keepclassmembers enum com.edgeray.** {
    public static **[] values();
    public static ** valueOf(java.lang.String);
}

# Keep Parcelable implementations
-keep class * implements android.os.Parcelable {
    public static final android.os.Parcelable$Creator *;
}

# Keep serializable classes
-keepclassmembers class * implements java.io.Serializable {
    static final long serialVersionUID;
    private static final java.io.ObjectStreamField[] serialPersistentFields;
    private void writeObject(java.io.ObjectOutputStream);
    private void readObject(java.io.ObjectInputStream);
    java.lang.Object writeReplace();
    java.lang.Object readResolve();
}

# Keep annotations
-keepattributes *Annotation*
-keepattributes Signature
-keepattributes Exceptions
-keepattributes InnerClasses
-keepattributes EnclosingMethod

# Keep source file names and line numbers for debugging
-keepattributes SourceFile,LineNumberTable

# Rename source file attribute to hide actual source file name
-renamesourcefileattribute SourceFile

# Remove logging in release builds
-assumenosideeffects class android.util.Log {
    public static *** d(...);
    public static *** v(...);
    public static *** i(...);
}

# Keep crash reporting
-keepattributes SourceFile,LineNumberTable
-keep public class * extends java.lang.Exception

# Optimize
-optimizationpasses 5
-dontusemixedcaseclassnames
-dontskipnonpubliclibraryclasses
-dontpreverify
-verbose

# Keep BuildConfig
-keep class com.edgeray.BuildConfig { *; }

# AndroidX
-keep class androidx.** { *; }
-dontwarn androidx.**

# Kotlin
-keep class kotlin.** { *; }
-keep class kotlin.Metadata { *; }
-dontwarn kotlin.**
-keepclassmembers class **$WhenMappings {
    <fields>;
}
-keepclassmembers class kotlin.Metadata {
    public <methods>;
}

# Coroutines
-keepnames class kotlinx.coroutines.internal.MainDispatcherFactory {}
-keepnames class kotlinx.coroutines.CoroutineExceptionHandler {}
-keepclassmembernames class kotlinx.** {
    volatile <fields>;
}

# OkHttp (if used)
-dontwarn okhttp3.**
-dontwarn okio.**
-keepnames class okhttp3.internal.publicsuffix.PublicSuffixDatabase

# Gson (if used)
-keepattributes Signature
-keepattributes *Annotation*
-dontwarn sun.misc.**
-keep class com.google.gson.** { *; }
-keep class * implements com.google.gson.TypeAdapterFactory
-keep class * implements com.google.gson.JsonSerializer
-keep class * implements com.google.gson.JsonDeserializer

# Prevent stripping of native libraries
-keep class com.edgeray.** {
    static {
        System.loadLibrary(...);
    }
}

# Keep native library loading
-keepclasseswithmembers class * {
    static {
        System.loadLibrary(...);
    }
}

# Specific to EdgeRay: Keep configuration classes
-keep class com.edgeray.config.** { *; }
-keep class com.edgeray.models.ServerConfig { *; }
-keep class com.edgeray.models.TunnelConfig { *; }

# Keep VPN-related classes
-keep class android.net.VpnService { *; }
-keep class android.net.VpnService$Builder { *; }

# Debugging: Print mapping to understand what's being obfuscated
-printmapping mapping.txt
-printseeds seeds.txt
-printusage usage.txt
