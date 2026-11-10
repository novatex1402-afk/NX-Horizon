# NX-Horizon: Android portu planı

Hedef: Horizon'u Android'e (APK) taşımak.

- **Arayüz aynı kalır.** Oyun içi ekranlar zaten oyun motorunun kendisi tarafından çiziliyor (wgpu). Launcher da
  masaüstündeki GPUI kodunun aynısı olarak çalışacak: GPUI'ye bir Android platformu yazıyoruz.
- **Dokunmatik kontroller** Minecraft PE'deki gibi: joystick veya D-pad, dokunarak ya da artı imleciyle nişan.
  Kod Horizon'da zaten var (`crates/horizon-client-render/src/touch*`, Starling'den, MIT); Android'de oyuna bağlanıyor.
- **Sürüm seçici:** dokunmatik kodun geldiği sürümden itibaren her Horizon sürümü Android için derlenir ve launcher'ın
  sürüm seçicisinde görünür.
- Mojang'ın dosyaları APK'ya konmaz: launcher, oyuncunun hesabıyla bunları Mojang'dan indirir (masaüstündeki gibi).

## Mimari

```
APK
├── LauncherActivity  (ana süreç)      GPUI launcher, gpui-android platformu üzerinde
│     └── sürüm seç → libhorizon-<sürüm>.so indir → launch-args.txt yaz → GameActivity başlat
└── GameActivity      (":game" süreci) NativeActivity; küçük bir yükleyici seçilen sürümün
      └── libhorizon.so                .so dosyasını açar ve android_main'i çağırır
```

- Launcher ile oyun ayrı süreçlerde çalışır: her biri kendi pencere ve olay döngüsünü alır.
  Masaüstünde de launcher oyunu ayrı bir program olarak başlatıyor.
- Android'de komut satırı yok. Launcher, oyunun argümanlarını uygulamanın dosya klasöründeki `launch-args.txt`
  dosyasına satır satır yazar (`crates/horizon-android`).

## Aşamalar

1. **Oyun Android'de derlenir ve çalışır** — devam ediyor
   - [x] Android SDK/NDK, `cargo ndk`
   - [x] Oyun kütüphanesi `aarch64-linux-android` için derleniyor (clipboard, DualSense, klavye düzeni düzeltmeleri)
   - [x] `crates/horizon-android`: `android_main`, winit'in Android olay döngüsü, argüman dosyası
   - [ ] `libhorizon.so` linkleniyor; APK paketleme (Gradle/AndroidManifest), ilk açılış bir cihazda/emülatörde
   - [ ] Dosya yolları, ses (AAudio), yüzey kaybı (uygulama arka plana gidince), ekran klavyesi
2. **Dokunmatik kontroller oyuna bağlanır**
   - [ ] Dokunma olayları → `TouchControls::update`, `poll_state` → oyunun kontrolcü girişi
   - [ ] Butonların çizimi (HUD üstünde), düzen düzenleyici ve ayarlar
3. **GPUI Android platformu (`gpui-android`)**
   - [ ] `Platform`, `PlatformWindow`, `PlatformDispatcher`: Android looper, ANativeWindow üzerinde `gpui_wgpu` çizici
   - [ ] Metin: `gpui_wgpu`'nun cosmic-text sistemi; dokunma → fare olayları, kaydırma hareketleri
   - [ ] Ekran klavyesi, panoya kopyalama, tarayıcıda URL açma (Microsoft girişi için)
4. **Launcher Android'de**
   - [ ] Masaüstüne özel kısımlar (süreç başlatma, kendini güncelleme, Java) Android'de karşılıklarıyla değişir
   - [ ] Mojang dosyalarının indirilip hazırlanması telefonda (Java olmadan)
5. **Sürümler**
   - [ ] CI: her Horizon sürümü için `libhorizon.so` (arm64-v8a, x86_64) derleyip release'e koyar
   - [ ] Launcher'ın sürüm seçicisi Android'de bu dosyaları listeler ve indirir

## Upstream

`upstream` uzak deposu `horizon-rust/horizon`. Güncellemeler `git fetch upstream && git merge upstream/main` ile alınır;
Android değişiklikleri küçük ve `cfg(target_os = "android")` arkasında tutulur, böylece birleştirmeler kolay kalır.
