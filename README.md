<div lang="fi">

# Mikä on <span lang="en">MathCAT</span>?

<span lang="en">MathCAT</span> muuntaa MathML-koodia puheeksi, pistekirjoitukseksi ja mahdollistaa liikkumisen matemaattisessa kaavassa. Suomenkielisen osuuden jälkeen tietoa enemmän englanniksi.

## Mikä tämä on?

Tämä on MathCATin suomenkielinen kehitysversio eli siinä on vielä paljon puutteita! Tällä hetkellä (9.11.2023) noin 32 % käännettävistä sanoista on käännetty englannista suomeksi. Puhe todennäköisesti toimii jo monille matemaattisille kaavoille, joita kohtaa opinnoissaan korkeakouluun asti. Puhe on käännetty suoraan englannista, joten tällä hetkellä monimutkaisimmat lauserakenteet kuulostavat oudoilta suomen kielellä. Tarkoituksena on kuitenkin vain ensin kääntää ja vasta sitten muuttaa lauserakenteita.

Kääntämisen jälkeen (vuonna 2024) lisätään pistekirjoitustuki pistenäytöille olemassa olevan [6-pisteen standardin mukaisesti (pistekirjoitus.fi)](https://www.pistekirjoitus.fi/julkaisut/matematiikka-ja-tietotekniikka/).

## Testaa suomenkielistä MathCAT-lisäosaa NVDA-ruudunlukijalla

Testaamiseen tarvitset NVDA-ruudunlukijan, joka on saatavilla vain Windows-käyttöjärjestelmälle. Ohje testaamiseen:

1. Lataa MathCAT-lisäosa NVDA:n lisäosakaupasta ja ota se käyttöön. Voit nyt kuunnella matemaattisia kaavoja englanniksi ja lukea ne myös pistenäytöltä Nemeth- tai UEB-standardilla.
2. Siirry painikkeeseen ”<span lang="en">code</span>”, paina valikko auki ja siirry sen sisällä kohtaan ”<span lang="en">Download zip</span>”.
3. Etsi ladattu tiedosto ja pura se. Avaa purettu kansio.
4. Siirry kansioon ”<span lang="en">Rules</span>” ja kopioi kansio ”<span lang="en">Languages</span>”.
5. Siirry sijaintiin, jonne NVDA lataa lisäosat. Alla ohjeet. Sen pitäisi olla muotoa ”\AppData\Roaming\nvda\addons\MathCAT\globalPlugins”.
   1. Windowsilla sinne pääsee kätevästi painamalla Win-näppäintä ja hakemalla ”Suorita”.
   2. Kirjoita Suorita-ikkunaan %appdata%. Se vie sinut oikeaan sijaintiin, josta löydät nvda-kansion (kirjoitettu pienellä).
   3. Etene siis nvda, addons, MathCAT, globalplugins, MathCAT, Rules. Rules-kansiossa on oma “Languages” -kansio.
   4. Liitä kansio ”Languages” ja hyväksy tiedostojen korvaaminen. Näin lisäät suomenkielisen osan MathCATiin.
6. Käynnistä NVDA varmuuden vuoksi uudelleen.
7. MathCATin asetuksia voi muuttaa NVDA-näppäin + N, asetukset, ”MathCAT settings”. (Valikko on englanninkielinen.) Voit muuttaa sitä kautta matematiikan luennan suomenkieliseksi.
8. Valitse luentatavaksi ”ClearSpeak”. Se on käännetty. ”SimpleSpeak” on myös suomenkielinen, mutta sen käännökset on tehty automaattisesti eli tulos ei ole kovin hyvä.
9. Voit nyt testata matematiikan kaavojen luentaa eri sivuilla, jos matematiikka on esitetty MathML-koodilla. [Kokeile esimerkiksi tekemääni testisivustoa](https://samimaattacelia.github.io/math-fi.html). Se ei ole kuitenkaan kattava, joten voit kokeilla esimerkiksi Wikipedian matematiikkasivuja. Niiss oleva matematiikka on esitetty MathML-koodilla.

Jos huomaat puutteita luennassa tai käännöksissä, niin voit lähettää palautetta suoraan Sami Määtälle, [sami.maatta@celia.fi](mailto:sami.maatta@celia.fi) tai lisätä GitHubin avulla "issuen".

</div>

## Edistyminen

- [ ] Tekstistä puheeksi suomeksi (32 %)
- [ ] Pistekirjoitustuki suomeksi (0 %)

## MathCAT: Math Capable Assistive Technology

<img alt="Logo. Text MathCAT with a cat sitting on the CAT part." src="logo.png" style="position: relative; top: 16px; z-index: -1;">
is a library that supports conversion of MathML to:

- Speech strings with embedded speech engine commands
- Braille (Nemeth, UEB Technical, and eventually other braille math codes)
- Navigation of math (in multiple ways including overviews)

There are four related projects that make use of MathCAT:

- [MathCATDemo](https://nsoiffer.github.io/MathCATDemo/) -- an online demonstration of some of what can be done with MathCAT
- [A python interface for MathCAT](https://github.com/NSoiffer/MathCATForPython) -- used by a [MathCAT NVDA add-on](https://addons.nvda-project.org/addons/MathCAT.en.html).
- [A C/C++ interface for MathCAT](https://github.com/NSoiffer/MathCATForC)
- [A Java interface for MathCAT](https://github.com/mwhapples/MathCAT4J) (thanks to Michael Whapples for working on that)

For more information, see the [full documentation](https://nsoiffer.github.io/MathCAT/).
