# micro-mechanics

drone mekaniği ve sürü algoritmaları için `no_std` rust kütüphanesi.

uçuş kontrolünden sürü zekasına kadar, drone'ların ihtiyaç duyduğu matematik ve kontrol algoritmalarını sıfırdan, bare-metal uyumlu şekilde yazıyorum.

## 🗺️ yol haritası (roadmap)

### faz 1: çekirdek motor (Core Flight Controller)
*donanıma en yakın, işletim sistemsiz (`no_std`) deterministik uçuş katmanı.*
- [x] matematik temeli (Vec3, Quaternion, PID)
- [x] motor mixer (Quad-X, satürasyon ve sınırlandırma)
- [ ] gelişmiş sensor füzyonu (Low-Pass Filter, Madgwick/Mahony)
- [ ] attitude controller (kaskad PID döngüleri)
- [ ] state machine & güvenlik (Arming, Failsafe, Watchdog)
- [ ] telemetri & loglama (hafif veri akışı altyapısı)

### faz 2: köprü ve ajan API (The Bridge)
*zeka algoritmaları ile uçuş motorunu donanımdan bağımsız bağlayan arayüz.*
- [ ] yörünge ve kinematik (`Vec3` hedefinden `Roll/Pitch` ivmesine dönüşüm)
- [ ] `Agent` trait API (algoritmalar için standart uçuş arayüzü)
- [ ] simülasyon adaptörü (algoritmaları bilgisayarda test etmek için mock arayüz)

### faz 3: sürü ve otonomi (Swarm & AI)
*çekirdek motor üzerinde çalışacak yüksek seviyeli karar mekanizmaları.*
- [ ] boids sürü algoritması (Separation, Alignment, Cohesion)
- [ ] stigmerji (sanal feromon ile dolaylı iletişim ve görev dağılımı)
- [ ] 3D pathfinding (engelli ortamlarda A* / D* Lite rotalama)
- [ ] formasyon uçuşu (dinamik lider-takipçi ağları)

### faz 4: agentic tinyML (Edge AI)
*kısıtlı MCU üzerinde makine öğrenmesi ve çıkarım (inference).*
- [ ] neural PID (havada dinamik katsayı öğrenen adaptif kontrol)
- [ ] micro-transformer çıkarımı (`no_std` on-device anomali tespiti)
- [ ] görsel odometri (optik akış ile kapalı alan konumlandırması)
