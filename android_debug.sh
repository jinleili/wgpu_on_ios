# build to Android target
cargo so b --target aarch64-linux-android 
cargo so b --target armv7-linux-androideabi
# copy .so files to jniLibs folder
cp target/aarch64-linux-android/debug/libwgpu_on_app.so android/app/libs/arm64-v8a/libwgpu_on_app.so
cp target/armv7-linux-androideabi/debug/libwgpu_on_app.so android/app/libs/armeabi-v7a/libwgpu_on_app.so