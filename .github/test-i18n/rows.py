# Translations for WordCraft's drop-down menu previews, written from the plain meaning of the
# English text (craft-i18n clean-room rule). Locale order as in craft-i18n/locales.
L = ["ar","bg","cs","da","de","el","es","et","fi","fr","hr","hu","id","it","ja","ka","kk","ko","lt","nl","pl","pt-br","ro","ru","sl","sr","sr-latn","sv","te","tr","uk","vi","zh-hans","zh-hant"]

def row(*vals):
    assert len(vals) == len(L), (len(vals), vals[:2])
    return dict(zip(L, vals))

PLAIN = {
"Numbering Library": row("مكتبة الترقيم","Библиотека за номериране","Knihovna číslování","Nummereringsbibliotek","Nummerierungsbibliothek","Βιβλιοθήκη αρίθμησης","Biblioteca de numeración","Nummerduste kogu","Numerointikirjasto","Bibliothèque de numérotation","Biblioteka numeriranja","Számozási könyvtár","Pustaka Penomoran","Raccolta di numerazioni","番号ライブラリ","ნუმერაციის ბიბლიოთეკა","Нөмірлеу кітапханасы","번호 매기기 라이브러리","Numeravimo biblioteka","Nummeringsbibliotheek","Biblioteka numeracji","Biblioteca de numeração","Bibliotecă de numerotare","Библиотека нумерации","Knjižnica oštevilčenja","Библиотека нумерисања","Biblioteka numerisanja","Numreringsbibliotek","సంఖ్యాంకన లైబ్రరీ","Numaralandırma Kitaplığı","Бібліотека нумерації","Thư viện đánh số","编号库","編號庫"),
"List Library": row("مكتبة القوائم","Библиотека със списъци","Knihovna seznamů","Listebibliotek","Listenbibliothek","Βιβλιοθήκη λιστών","Biblioteca de listas","Loendite kogu","Luettelokirjasto","Bibliothèque de listes","Biblioteka popisa","Listakönyvtár","Pustaka Daftar","Raccolta di elenchi","リスト ライブラリ","სიების ბიბლიოთეკა","Тізімдер кітапханасы","목록 라이브러리","Sąrašų biblioteka","Lijstbibliotheek","Biblioteka list","Biblioteca de listas","Bibliotecă de liste","Библиотека списков","Knjižnica seznamov","Библиотека листа","Biblioteka lista","Listbibliotek","జాబితా లైబ్రరీ","Liste Kitaplığı","Бібліотека списків","Thư viện danh sách","列表库","清單庫"),
"Rectangles": row("المستطيلات","Правоъгълници","Obdélníky","Rektangler","Rechtecke","Ορθογώνια","Rectángulos","Ristkülikud","Suorakulmiot","Rectangles","Pravokutnici","Téglalapok","Persegi Panjang","Rettangoli","四角形","მართკუთხედები","Тіктөртбұрыштар","사각형","Stačiakampiai","Rechthoeken","Prostokąty","Retângulos","Dreptunghiuri","Прямоугольники","Pravokotniki","Правоугаоници","Pravougaonici","Rektanglar","దీర్ఘచతురస్రాలు","Dikdörtgenler","Прямокутники","Hình chữ nhật","矩形","矩形"),
"Basic Shapes": row("الأشكال الأساسية","Основни фигури","Základní tvary","Grundlæggende figurer","Grundformen","Βασικά σχήματα","Formas básicas","Põhikujundid","Perusmuodot","Formes de base","Osnovni oblici","Alapformák","Bentuk Dasar","Forme di base","基本図形","ძირითადი ფიგურები","Негізгі фигуралар","기본 도형","Pagrindinės figūros","Basisvormen","Kształty podstawowe","Formas básicas","Forme de bază","Основные фигуры","Osnovne oblike","Основни облици","Osnovni oblici","Grundformer","ప్రాథమిక ఆకారాలు","Temel Şekiller","Основні фігури","Hình cơ bản","基本形状","基本形狀"),
"Block Arrows": row("أسهم عريضة","Плътни стрелки","Blokové šipky","Blokpile","Blockpfeile","Συμπαγή βέλη","Flechas de bloque","Plokknooled","Lohkonuolet","Flèches pleines","Blok strelice","Tömbnyilak","Panah Blok","Frecce a blocco","ブロック矢印","ბლოკური ისრები","Блоктық көрсеткілер","블록 화살표","Blokinės rodyklės","Blokpijlen","Strzałki blokowe","Setas em bloco","Săgeți bloc","Блочные стрелки","Bločne puščice","Блок стрелице","Blok strelice","Blockpilar","బ్లాక్ బాణాలు","Blok Oklar","Блокові стрілки","Mũi tên khối","块状箭头","區塊箭頭"),
"Flowchart": row("مخطط انسيابي","Блокова схема","Vývojový diagram","Rutediagram","Flussdiagramm","Διάγραμμα ροής","Diagrama de flujo","Plokkskeem","Vuokaavio","Organigramme","Dijagram toka","Folyamatábra","Diagram Alir","Diagramma di flusso","フローチャート","ბლოკ-სქემა","Блок-схема","순서도","Struktūrinė schema","Stroomdiagram","Schemat blokowy","Fluxograma","Diagramă de flux","Блок-схема","Diagram poteka","Дијаграм тока","Dijagram toka","Flödesschema","ఫ్లోచార్ట్","Akış Şeması","Блок-схема","Lưu đồ","流程图","流程圖"),
"Stars and Banners": row("نجوم ولافتات","Звезди и ленти","Hvězdy a stuhy","Stjerner og bannere","Sterne und Banner","Αστέρια και κορδέλες","Estrellas y banderolas","Tähed ja lindid","Tähdet ja nauhat","Étoiles et bannières","Zvijezde i trake","Csillagok és szalagok","Bintang dan Spanduk","Stelle e nastri","星とリボン","ვარსკვლავები და ლენტები","Жұлдыздар мен таспалар","별 및 배너","Žvaigždės ir juostos","Sterren en banners","Gwiazdy i wstęgi","Estrelas e faixas","Stele și banere","Звезды и ленты","Zvezde in trakovi","Звезде и траке","Zvezde i trake","Stjärnor och banderoller","నక్షత్రాలు మరియు బ్యానర్లు","Yıldızlar ve Şeritler","Зірки та стрічки","Sao và biểu ngữ","星形和横幅","星形和橫幅"),
"Right Triangle": row("مثلث قائم الزاوية","Правоъгълен триъгълник","Pravoúhlý trojúhelník","Retvinklet trekant","Rechtwinkliges Dreieck","Ορθογώνιο τρίγωνο","Triángulo rectángulo","Täisnurkne kolmnurk","Suorakulmainen kolmio","Triangle rectangle","Pravokutni trokut","Derékszögű háromszög","Segitiga Siku-siku","Triangolo rettangolo","直角三角形","მართკუთხა სამკუთხედი","Тікбұрышты үшбұрыш","직각 삼각형","Statusis trikampis","Rechthoekige driehoek","Trójkąt prostokątny","Triângulo retângulo","Triunghi dreptunghic","Прямоугольный треугольник","Pravokotni trikotnik","Правоугли троугао","Pravougli trougao","Rätvinklig triangel","లంబకోణ త్రిభుజం","Dik Üçgen","Прямокутний трикутник","Tam giác vuông","直角三角形","直角三角形"),
"Flowchart: Or": row("مخطط انسيابي: أو","Блокова схема: или","Vývojový diagram: nebo","Rutediagram: eller","Flussdiagramm: Oder","Διάγραμμα ροής: Ή","Diagrama de flujo: o","Plokkskeem: või","Vuokaavio: tai","Organigramme : ou","Dijagram toka: ili","Folyamatábra: vagy","Diagram Alir: Atau","Diagramma di flusso: o","フローチャート: または","ბლოკ-სქემა: ან","Блок-схема: немесе","순서도: 또는","Struktūrinė schema: arba","Stroomdiagram: of","Schemat blokowy: lub","Fluxograma: ou","Diagramă de flux: sau","Блок-схема: или","Diagram poteka: ali","Дијаграм тока: или","Dijagram toka: ili","Flödesschema: eller","ఫ్లోచార్ట్: లేదా","Akış Şeması: Veya","Блок-схема: або","Lưu đồ: Hoặc","流程图：或","流程圖：或"),
"Arrow: Right": row("سهم: لليمين","Стрелка: надясно","Šipka: doprava","Pil: højre","Pfeil: nach rechts","Βέλος: δεξιά","Flecha: derecha","Nool: paremale","Nuoli: oikealle","Flèche : droite","Strelica: desno","Nyíl: jobbra","Panah: Kanan","Freccia: destra","矢印: 右","ისარი: მარჯვნივ","Көрсеткі: оңға","화살표: 오른쪽","Rodyklė: dešinėn","Pijl: rechts","Strzałka: w prawo","Seta: direita","Săgeată: dreapta","Стрелка: вправо","Puščica: desno","Стрелица: надесно","Strelica: nadesno","Pil: höger","బాణం: కుడివైపు","Ok: Sağ","Стрілка: праворуч","Mũi tên: Phải","箭头：右","箭頭：右"),
"Arrow: Left": row("سهم: لليسار","Стрелка: наляво","Šipka: doleva","Pil: venstre","Pfeil: nach links","Βέλος: αριστερά","Flecha: izquierda","Nool: vasakule","Nuoli: vasemmalle","Flèche : gauche","Strelica: lijevo","Nyíl: balra","Panah: Kiri","Freccia: sinistra","矢印: 左","ისარი: მარცხნივ","Көрсеткі: солға","화살표: 왼쪽","Rodyklė: kairėn","Pijl: links","Strzałka: w lewo","Seta: esquerda","Săgeată: stânga","Стрелка: влево","Puščica: levo","Стрелица: налево","Strelica: nalevo","Pil: vänster","బాణం: ఎడమవైపు","Ok: Sol","Стрілка: ліворуч","Mũi tên: Trái","箭头：左","箭頭：左"),
"Arrow: Up": row("سهم: للأعلى","Стрелка: нагоре","Šipka: nahoru","Pil: op","Pfeil: nach oben","Βέλος: πάνω","Flecha: arriba","Nool: üles","Nuoli: ylös","Flèche : haut","Strelica: gore","Nyíl: fel","Panah: Atas","Freccia: su","矢印: 上","ისარი: ზემოთ","Көрсеткі: жоғары","화살표: 위쪽","Rodyklė: aukštyn","Pijl: omhoog","Strzałka: w górę","Seta: para cima","Săgeată: sus","Стрелка: вверх","Puščica: gor","Стрелица: нагоре","Strelica: nagore","Pil: upp","బాణం: పైకి","Ok: Yukarı","Стрілка: угору","Mũi tên: Lên","箭头：上","箭頭：上"),
"Arrow: Down": row("سهم: للأسفل","Стрелка: надолу","Šipka: dolů","Pil: ned","Pfeil: nach unten","Βέλος: κάτω","Flecha: abajo","Nool: alla","Nuoli: alas","Flèche : bas","Strelica: dolje","Nyíl: le","Panah: Bawah","Freccia: giù","矢印: 下","ისარი: ქვემოთ","Көрсеткі: төмен","화살표: 아래쪽","Rodyklė: žemyn","Pijl: omlaag","Strzałka: w dół","Seta: para baixo","Săgeată: jos","Стрелка: вниз","Puščica: dol","Стрелица: надоле","Strelica: nadole","Pil: ned","బాణం: కిందికి","Ok: Aşağı","Стрілка: униз","Mũi tên: Xuống","箭头：下","箭頭：下"),
"Simple Frame": row("إطار بسيط","Обикновена рамка","Jednoduchý rámeček","Enkel ramme","Einfacher Rahmen","Απλό πλαίσιο","Marco simple","Lihtne raam","Yksinkertainen kehys","Cadre simple","Jednostavan okvir","Egyszerű keret","Bingkai Sederhana","Cornice semplice","シンプルな枠","მარტივი ჩარჩო","Қарапайым жақтау","단순한 테두리","Paprastas rėmelis","Eenvoudig kader","Prosta ramka","Moldura simples","Ramă simplă","Простая рамка","Preprost okvir","Једноставан оквир","Jednostavan okvir","Enkel ram","సాధారణ ఫ్రేమ్","Basit Çerçeve","Проста рамка","Khung đơn giản","简单边框","簡單邊框"),
"Thick Frame": row("إطار سميك","Дебела рамка","Silný rámeček","Tyk ramme","Dicker Rahmen","Παχύ πλαίσιο","Marco grueso","Paks raam","Paksu kehys","Cadre épais","Debeli okvir","Vastag keret","Bingkai Tebal","Cornice spessa","太い枠","სქელი ჩარჩო","Қалың жақтау","두꺼운 테두리","Storas rėmelis","Dik kader","Gruba ramka","Moldura grossa","Ramă groasă","Толстая рамка","Debel okvir","Дебео оквир","Debeo okvir","Tjock ram","మందమైన ఫ్రేమ్","Kalın Çerçeve","Товста рамка","Khung dày","粗边框","粗邊框"),
"Rounded": row("زوايا مستديرة","Заоблени ъгли","Zaoblené rohy","Afrundede hjørner","Abgerundete Ecken","Στρογγυλεμένες γωνίες","Esquinas redondeadas","Ümarad nurgad","Pyöristetyt kulmat","Coins arrondis","Zaobljeni kutovi","Lekerekített sarkok","Sudut Membulat","Angoli arrotondati","角丸","მომრგვალებული კუთხეები","Дөңгеленген бұрыштар","둥근 모서리","Užapvalinti kampai","Afgeronde hoeken","Zaokrąglone rogi","Cantos arredondados","Colțuri rotunjite","Скругленные углы","Zaobljeni koti","Заобљени углови","Zaobljeni uglovi","Rundade hörn","గుండ్రని మూలలు","Yuvarlatılmış Köşeler","Заокруглені кути","Góc bo tròn","圆角","圓角"),
"Soft Edge": row("حواف ناعمة","Меки ръбове","Měkké okraje","Bløde kanter","Weiche Kanten","Απαλές άκρες","Bordes suaves","Pehmed servad","Pehmeät reunat","Bords adoucis","Meki rubovi","Lágy szélek","Tepi Lembut","Bordi sfumati","輪郭のぼかし","რბილი კიდეები","Жұмсақ жиектер","부드러운 가장자리","Švelnūs kraštai","Zachte randen","Zmiękczone krawędzie","Bordas suaves","Margini estompate","Сглаживание","Mehki robovi","Меке ивице","Meke ivice","Mjuka kanter","మృదువైన అంచులు","Yumuşak Kenarlar","М'які краї","Cạnh mềm","柔化边缘","柔化邊緣"),
"Change to Level {n}": row("تغيير إلى المستوى {n}","Промяна на ниво {n}","Změnit na úroveň {n}","Skift til niveau {n}","Auf Ebene {n} ändern","Αλλαγή σε επίπεδο {n}","Cambiar al nivel {n}","Muuda tasemeks {n}","Vaihda tasolle {n}","Passer au niveau {n}","Promijeni u razinu {n}","Váltás {n}. szintre","Ubah ke Tingkat {n}","Passa al livello {n}","レベル {n} に変更","დონე {n}-ზე შეცვლა","{n}-деңгейге өзгерту","수준 {n}(으)로 변경","Keisti į {n} lygį","Wijzigen in niveau {n}","Zmień na poziom {n}","Alterar para o nível {n}","Schimbare la nivelul {n}","Изменить на уровень {n}","Spremeni v raven {n}","Промени у ниво {n}","Promeni u nivo {n}","Ändra till nivå {n}","స్థాయి {n}కి మార్చు","Düzey {n} olarak değiştir","Змінити на рівень {n}","Đổi sang cấp {n}","更改为级别 {n}","變更為層級 {n}"),
"Thin": row("رفيع","Тънка","Tenká","Tynd","Dünn","Λεπτό","Fino","Õhuke","Ohut","Fin","Tanko","Vékony","Tipis","Sottile","細い","თხელი","Жіңішке","얇게","Plonas","Dun","Cienka","Fina","Subțire","Тонкая","Tanko","Танко","Tanko","Tunn","సన్నని","İnce","Тонка","Mỏng","细","細"),
}

# Insert › Shapes, worded for the gallery (`shapes` context): drawn lines and figures, not lines
# of text. Added only where the plain translation is missing or means something else.
SHAPES = {
"Lines": row("خطوط","Линии","Čáry","Linjer","Linien","Γραμμές","Líneas","Jooned","Viivat","Lignes","Crte","Vonalak","Garis","Linee","線","ხაზები","Сызықтар","선","Linijos","Lijnen","Linie","Linhas","Linii","Линии","Črte","Линије","Linije","Linjer","గీతలు","Çizgiler","Лінії","Đường kẻ","线条","線條"),
}
SHAPES_FIX = {
    ("tr", "Line"): "Çizgi",
    ("ko", "Line"): "선",
    ("ka", "Diamond"): "რომბი",
    ("sr", "Diamond"): "Ромб",
    ("sr-latn", "Diamond"): "Romb",
    ("da", "Diamond"): "Rombe",
    ("ar", "Rounded Rectangle"): "مستطيل مستدير الزوايا",
    ("kk", "Rounded Rectangle"): "Дөңгеленген тіктөртбұрыш",
    ("lt", "Rounded Rectangle"): "Suapvalintas stačiakampis",
    ("te", "Rounded Rectangle"): "గుండ్రని మూలల దీర్ఘచతురస్రం",
    ("ar", "Triangle"): "مثلث",
    ("kk", "Triangle"): "Үшбұрыш",
    ("lt", "Triangle"): "Trikampis",
    ("te", "Triangle"): "త్రిభుజం",
    ("ar", "Heart"): "قلب",
    ("fi", "Heart"): "Sydän",
    ("kk", "Heart"): "Жүрек",
    ("lt", "Heart"): "Širdis",
    ("te", "Heart"): "హృదయం",
}
CONNECTORS = ["Straight Connector", "Straight Arrow Connector", "Elbow Connector", "Elbow Arrow Connector", "Curved Connector", "Curved Arrow Connector"]
CONNECTOR_FILL = {
"ar": ["موصل مستقيم","موصل مستقيم بسهم","موصل بزاوية","موصل بزاوية بسهم","موصل منحنٍ","موصل منحنٍ بسهم"],
"el": ["Ευθεία σύνδεση","Ευθεία σύνδεση με βέλος","Γωνιώδης σύνδεση","Γωνιώδης σύνδεση με βέλος","Καμπύλη σύνδεση","Καμπύλη σύνδεση με βέλος"],
"fi": ["Suora yhdysviiva","Suora yhdysviiva nuolella","Kulmikas yhdysviiva","Kulmikas yhdysviiva nuolella","Kaareva yhdysviiva","Kaareva yhdysviiva nuolella"],
"id": ["Penghubung Lurus","Penghubung Lurus Berpanah","Penghubung Siku","Penghubung Siku Berpanah","Penghubung Lengkung","Penghubung Lengkung Berpanah"],
"kk": ["Түзу қосқыш","Көрсеткісі бар түзу қосқыш","Бұрышты қосқыш","Көрсеткісі бар бұрышты қосқыш","Қисық қосқыш","Көрсеткісі бар қисық қосқыш"],
"lt": ["Tiesi jungtis","Tiesi jungtis su rodykle","Laužta jungtis","Laužta jungtis su rodykle","Lenkta jungtis","Lenkta jungtis su rodykle"],
"te": ["సరళ కనెక్టర్","బాణంతో సరళ కనెక్టర్","కోణ కనెక్టర్","బాణంతో కోణ కనెక్టర్","వక్ర కనెక్టర్","బాణంతో వక్ర కనెక్టర్"],
}
for loc, vals in CONNECTOR_FILL.items():
    for k, v in zip(CONNECTORS, vals):
        SHAPES_FIX[(loc, k)] = v

# The numbering library's Roman numerals: list numbers, the same in every language (kept as is).
for numeral in ["II.", "III.", "ii.", "iii."]:
    PLAIN[numeral] = {loc: numeral for loc in L}
