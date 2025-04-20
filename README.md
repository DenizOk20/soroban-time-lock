# ⏱️ Soroban Time Lock - Claimable Balance Smart Contract
 
Kullanıcılar için **zaman kilitli token** göndermeyi ve sadece belirli bir süreden sonra **claim** etmeyi mümkün kılar.

## 📦 Özellikler

- ✅ Belirli bir `token`, `miktar` ve `claimant` adresleri ile zaman kilidi oluşturur  
- ⏳ Belirlenen süreden önce claim yapılamaz  
- 🔐 Tek bir adres veya çoklu claimant desteklenir  
- 🧪 Test senaryoları tam kapsamlıdır  

## 🛠️ Kullanım

### Contract'ı deploy et
```bash
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/soroban_timelock.wasm
```

### Test için
```bash
cargo test
```
Testlerde zaman ayarı, farklı claim denemeleri ve edge case'ler bulunur.
