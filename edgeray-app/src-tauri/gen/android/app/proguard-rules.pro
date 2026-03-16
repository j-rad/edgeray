# ProGuard rules for EdgeRay Android App

# Keep UniFFI generated classes
-keep class uniffi.** { *; }

# Keep Juniper (GraphQL) classes if used generically
-keep class juniper.** { *; }

# Keep our own JNI bridge classes
-keep class com.jrad.edgeray_app.** { *; }

# Keep standard Android components
-keep public class * extends android.app.Activity
-keep public class * extends android.app.Application
-keep public class * extends android.app.Service

# Preserve annotations
-keepattributes *Annotation*
-keepattributes Signature