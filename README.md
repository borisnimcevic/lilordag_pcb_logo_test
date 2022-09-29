## The PCB
Color/shades options and combinations (for the green pcb):
* silkscreen (white)
* copper with no solder mask (gold or silver)
* copper with solder mask (light green)
* solder mask without copper under (dark green)
* no solder mask and no copper (FR4 collor)


| foreground \ background | white | gold/silver | dark green | light green | clear |
| -------------           |-------| ----------- | ---------- |-------------|-------|
| white                   | X     |             |            |             |       |
| gold/silver             |       |  X          |            |             |       |
| dark green              |       |             |       X    |             |       |
| light green             |       |             |            | X           |       |
| clear                   |       |             |            |             | X     |


## Bill Of Materials

### Astable Multivibrator
|Component  |Quantity |Link   |
|-----      |-----    |-----  |
|Ceramic Capacitor 0603 470nF   |2 |[digikey](https://www.digikey.se/en/products/detail/samsung-electro-mechanics/CL10B474KA8NNWC/3887742) |
|Resistor 0603 470k|2|[digikey](https://www.digikey.se/en/products/detail/stackpole-electronics-inc/RMCF0603FT470K/1761140) |
|Resistor 0603 470|4 |[digikey](https://www.digikey.se/en/products/detail/vishay-beyschlag-draloric-bc-components/MCT0603MD4700DP500/2092094) |
|Transistor SOT-23 BC846|2|[digikey](https://www.digikey.se/en/products/detail/nexperia-usa-inc/BC846B-235/1232263) |
|Red LED 0603|2|[digikey](https://www.digikey.se/en/products/detail/ams-osram-usa-inc/LS-Q976-NR-1/1227986) |
|Slide switch|3|[digikey](https://www.digikey.se/en/products/detail/c-k/PCM12SMTR/1640112?utm_medium=aggregator&utm_source=snapeda&utm_campaign=buynow) |

### ESP32-C3 shenanigans
|Component  |Quantity |Link   |
|-----      |-----    |-----  |
|ESP32-C3|1 |[digikey](https://www.digikey.se/en/products/detail/espressif-systems/ESP32-C3-WROOM-02-N4/14553031) |

### Common
|Component  |Quantity |Link   |
|-----      |-----    |-----  |
|LDO 3.3V 600mA|1        |[digikey](https://www.digikey.se/en/products/detail/diodes-incorporated/AP7366-33W5-7/9867322) |
|MMBT3906-7-F PNP Transistor|2        |[digikey](https://www.digikey.se/en/products/detail/diodes-incorporated/MMBT3906-7-F/770797) |
|Transistor SOT-23 BC846|2|[digikey](https://www.digikey.se/en/products/detail/nexperia-usa-inc/BC846B-235/1232263) |
|USB-C|1        |[digikey](https://www.digikey.se/en/products/detail/gct/USB4105-GF-A/11198441) |
|CR2032 battery holder - BK-912-TR|2        |[digikey](https://www.digikey.se/en/products/detail/mpd-memory-protection-devices/BK-912-TR/2077831?s=N4IgTCBcDaIEIGkC0BOAjGJBhAKkgcgCIgC6AvkA) |
|P-Channel MOSFET SSM3J56MFV,L3F|1        |[digikey](https://www.digikey.se/en/products/detail/toshiba-semiconductor-and-storage/SSM3J56MFV-L3F/4305160) |
|Zener diode MMSZ5233B-AU_R1_000A1|1        |[digikey](https://www.digikey.se/en/products/detail/panjit-international-inc/MMSZ5233B-AU-R1-000A1/15796616) |
