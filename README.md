# Format DocuWiki

==== Projekt: Jednowymiarowy problem pakowania (1D-BPP) ====

^ Data ^ Status projektu    ^ Uwagi ^
|2020-11-14|Wybór tematu    | |
|2022-11-14|Rozpoczęty      | |
|2022-12-12|Recenzowany     | |

Maciej Kaszkowiak, 151856

=== Recenzent ===

==== O problemie ====
Jednowymiarowy problem pakowania (1D bin packing problem) to rodzaj problemu optymalizacyjnego, który polega na próbie zapakowania zbioru przedmiotów o różnych rozmiarach do stałej liczby kontenerów, tak aby zminimalizować ich liczbę. W jednowymiarowym pakowaniu do pojemników, każdy przedmiot jest reprezentowany przez pojedynczą liczbę wskazującą jego rozmiar, zakłada się również, że pojemniki mają jednolity rozmiar. Celem jest znalezienie optymalnego sposobu pakowania przedmiotów do kontenerów, tak aby żaden kontener nie był przepełniony, a liczba użytych kontenerów była minimalna. Problem ten jest ważny w wielu praktycznych zastosowaniach, m.in. w alokacji zasobów i harmonogramowaniu.

Jednowymiarowy problem pakowania pojemników jest NP-trudny, co oznacza, że dla dużych zbiorów przedmiotów znalezienie optymalnego rozwiązania problemu jest bardzo czasochłonne lub praktycznie niewykonalne. Istnieją jednak przybliżone algorytmy i heurystyki, które można wykorzystać do znalezienia dobrych, choć niekoniecznie optymalnych, rozwiązań problemu w rozsądnym czasie.

=== Algorytmy online a offline ===
W wariancie offline wszystkie elementy są na początku znane, przez co możemy je posortować według rozmiaru przed uruchomieniem algorytmu. Dzięki temu algorytm może podejmować bardziej efektywne decyzje o tym, które elementy powinny być umieszczone w których pojemnikach. 

W wariancie online elementy nie mogą zostać posortowane przed uruchomieniem algorytmu. Oznacza to, że algorytm musi podejmować decyzje o tym, które elementy powinny być umieszczone w których koszach bez korzyści wynikających z wcześniejszej znajomości rozmiarów elementów. Prowadzi to do gorszej średniej dokładności algorytmu.

=== Next Fit (NF) ===

Algorytm Next Fit zawsze utrzymuje jeden otwarty kontener i gdy nowy element nie mieści się w nim, zamyka bieżący kontener i otwiera nowy. Klasa implementuje metodę solve(), która przyjmuje na wejście wektor elementów i zwraca wektor pojemników. Metoda ta iteruje przez elementy i próbuje dodać je do ostatniego pojemnika z listy. Jeśli element się nie mieści, tworzony jest nowy pojemnik i element jest do niego dodawany.

<code rust>
impl Solver for SolverNextFit {
    /* Next Fit (NF) always keeps a single open bin. 
    When the new item does not fit into it, it closes the current bin and opens a new bin */

    fn solve(&self, input: Vec<Item>) -> Vec<Container> {
        let mut results: Vec<Container> = Vec::new();
        let mut last_index: usize = 0;
        let first_container: Container = self.new_container();
        results.push(first_container);

        for item in input {
            // Add the item to the last container from the list
            let rejected_item: Option<Item> = results.get_mut(last_index).unwrap().add(item);
            if rejected_item.is_none() {
                continue;  
            }
            
            // If the container won't fit the item, create a new one
            let mut new_container: Container = self.new_container();
            let rejected_item_from_new_container: Option<Item> = new_container.add(rejected_item.unwrap());

            // If a new container can't fit the item, panic. 
            // Won't be able to provide a solution
            if !rejected_item_from_new_container.is_none() {
                panic!("An item won't fit into an empty container!");
            }
            
            results.push(new_container);
            last_index += 1;
        }

        return results;
    }
}
</code>

=== First Fit (FF) ===
Algorytm First Fit utrzymuje wszystkie pojemniki otwarte w kolejności, w jakiej zostały otwarte, i próbuje umieścić każdy nowy element w pierwszym pojemniku, w którym pasuje. Jeśli element nie zmieści się w pierwszym pojemniku, algorytm utworzy nowy pojemnik i umieści w nim element.

Algorytm First Fit różni się od algorytmu Next Fit pod względem złożoności obliczeniowej. Algorytm Next Fit patrzy tylko na ostatni otwarty kosz, natomiast algorytm First Fit patrzy na wszystkie kosze w kolejności. Oznacza to, że algorytm First Fit ma większą złożoność obliczeniową niż algorytm Next Fit. Złożoność czasowa algorytmu First Fit wynosi $O(n^2)$ w przypadku mojej implementacji, natomiast możliwa jest również implementacja w czasie $O(n \log n)$ za pomocą drzewa binarnego. 

<code rust>
impl Solver for SolverFirstFit {
    /* First-Fit (FF) keeps all bins open, in the order in which they were opened. 
    It attempts to place each new item into the first bin in which it fits. */
 
    fn solve(&self, input: Vec<Item>) -> Vec<Container> {
        let mut results: Vec<Container> = Vec::new();
        let mut containers_count: usize = 1;
        let first_container: Container = self.new_container();
        results.push(first_container);

        for item in input {
            // Add the item to the last container from the list
            let mut rejected_item: Option<Item> = Some(item);
            for index in 0..containers_count {
                rejected_item = results.get_mut(index).unwrap().add(rejected_item.unwrap());
                if rejected_item.is_none() {
                    break;
                }
            }
            
            if rejected_item.is_none() {
                continue;
            }

            // If the container won't fit the item, create a new one
            let mut new_container: Container = self.new_container();
            let rejected_item_from_new_container: Option<Item> = new_container.add(rejected_item.unwrap());

            // If a new container can't fit the item, panic. 
            // Won't be able to provide a solution
            if !rejected_item_from_new_container.is_none() {
                panic!("An item won't fit into an empty container!");
            }
            
            results.push(new_container);
            containers_count += 1;
        }

        return results;
    }
}
</code>

== Metodologia ==
Kod został zaimplementowany z wykorzystaniem języka Rust.

Dane zostały wygenerowane dla wszystkich dostępnych permutacji zaimplementowanych heurystyk oraz ustawień problemu. 

Testy wydajnościowe zostały przeprowadzone 10 razy dla każdej permutacji przy różnych wygenerowanych danych. Celem było zmniejszenie wpływu losowości na zmierzoną dokładność algorytmu oraz zmniejszenie wpływu obciążenia komputera na prędkość wykonywania. 

Ustawienia problemu obejmowały następujące 4 parametry:
- minimalny rozmiar przedmiotu (item_size_min)
- maksymalny rozmiar przedmiotu (item_size_max)
- liczba wygenerowanych przedmiotów (item_limit)
- rozmiar pojedynczego kontenera (container_size)

Wykorzystałem następujące ustawienia problemu.
- item_size_min = {1, 50, 100, 500} dla item_size_max = 1000 oraz item_limit = 10000
- item_size_max = {100, 500, 1000} dla item_size_min = 1 oraz item_limit = 10000
- item_limit = {10000, 5000, 1000, 500, 100, 50} dla item_size_min = 1 oraz item_size_max = 1000

Dla każdego ustawienia rozmiar kontenera wynosił 10000 jednostek.

Poszczególne ustawienia miały na celu zmierzenie wpływu zmiany rodzaju dystrybucji przedmiotów na wydajność oraz optymalność testowanych algorytmów.

Zbadałem następujące algorytmy:
- Next Fit [online]
- First Fit [online]
- Next Fit Decreasing [offline]
- First Fit Decreasing [offline]

Czas wykonywania został zmierzony z dokładnością do mikrosekund.

Program akceptuje dane wejściowe oraz zwraca odpowiedź w formacie JSON.

Jakość rozwiązania została określona jako iloraz liczby kontenerów w rozwiązaniu i rozwiązaniu optymalnym. Przykładowo - rozwiązanie optymalne ma jakość 1, rozwiązanie generujące o dwa razy za dużo kontenerów ma jakość 2.

=== Losowy generator danych wejściowych ===

Generator działa poprzez losowe generowanie elementów o rozmiarach pomiędzy minimalnym a maksymalnym rozmiarem określonym w ustawieniach. Następnie sprawdza, czy rozmiar elementu plus aktualny rozmiar pojemnika przekraczają maksymalny rozmiar pojemnika. Jeśli tak, ustawia rozmiar elementu na największy możliwy do zmieszczenia w pojemniku. Następnie tasuje elementy i zwraca elementy oraz optymalną liczbę pojemników.

<code rust>
impl Generator {
    fn generate(&self) -> GeneratorResults {
        let mut current_size = 0;
        let mut containers = 0;

        let mut items: Vec<Item> = Vec::new();
        for n in 0..self.settings.item_limit {
            if current_size == 0 {
                containers += 1;
            }

            let mut size: u32 = rand::thread_rng().gen_range(
                self.settings.item_size_min..self.settings.item_size_max
            );

            let item_overflows_current_container: bool = (current_size + size) > self.settings.container_size;
            let last_item_to_generate: bool = n == self.settings.item_limit - 1;

            if item_overflows_current_container || last_item_to_generate {
                let biggest_item_possible_to_fit: u32 = self.settings.container_size - current_size;
                size = biggest_item_possible_to_fit;
            }

            items.push(Item {
                size: size 
            });

            current_size = (current_size + size) % self.settings.container_size;
        }

        items.shuffle(&mut thread_rng());

        return GeneratorResults {
            items: items,
            optimal_container_count: containers
        };
    }
}
</code>

=== Format danych wejściowych ===
<code json>
{
    "solvers": [
        {"id": "First Fit", "sorted": true},
        {"id": "First Fit", "sorted": false},
        {"id": "Next Fit", "sorted": true},
        {"id": "Next Fit", "sorted": false}
    ],

    "settings": [
        {
            "item_size_min": 1,
            "item_size_max": 1000,
            "item_limit": 10000,
            "container_size": 1000
        },
        {
            "item_size_min": 50,
            "item_size_max": 1000,
            "item_limit": 10000,
            "container_size": 1000
        }
    ],

    "iterations": 10
}
</code>


=== Wyniki ===

{{ :ok22:s151856:output.png |}}

=== Zestawienie wyników ===

Dokładność heurystyk dla jednowymiarowego problemu pakowania może być bardzo różna w zależności od rozkładu rozmiarów przedmiotów, które należy zapakować. W niektórych przypadkach algorytm heurystyczny może być w stanie znaleźć rozwiązanie bardzo zbliżone do rozwiązania optymalnego lub nawet osiągnąć rozwiązanie optymalne. 

W większości przypadków wydajność algorytmów heurystycznych może być znacznie gorsza. Znalezienie dobrego rozwiązania pakowania może być trudne dla algorytmu heurystycznego, a znalezione przez niego rozwiązanie może nie być bliskie rozwiązaniu optymalnemu.

Ogólnie rzecz biorąc, należy pamiętać, że algorytmy heurystyczne nie gwarantują znalezienia optymalnego rozwiązania jednowymiarowego problemu pakowania pojemników. Ich zasada działania opiera się o szybkim szukaniu w miarę dobrych rozwiązań, które niekoniecznie muszą być w pełni optymalne.

==== Optymalność rozwiązań w zależności od minimalnego rozmiaru przedmiotów ====
{{ :ok22:s151856:wydajnosc1.png?900 |}}

==== Optymalność rozwiązań w zależności od maksymalnego rozmiaru przedmiotów ====
{{ :ok22:s151856:wydajnosc2.png?900 |}}

==== Optymalność rozwiązań w zależności od limitu przedmiotów ====
{{ :ok22:s151856:wydajnosc3.png?900 |}}

==== Optymalność rozwiązań dla wszystkich permutacji liczby przedmiotów i zakresu wartości ====
{{ :ok22:s151856:wydajnosc4.png?900 |}}

==== Wydajność algorytmów w zależności od limitu przedmiotów ====

Dla czytelności na pierwszym wykresie zastosowano skalę logarytmiczną:

{{ :ok22:s151856:wydajnosc5.png?900 |}}
{{ :ok22:s151856:wydajnosc6.png?900 |}}


=== Wnioski ===

Możemy zauważyć, że:

1. Algorytmy offline generują bardziej optymalne rozwiązania w porównaniu do algorytmów online.

2. Algorytmy offline zajmują więcej czasu w porównaniu do algorytmów offline ze względu na dodatkowy czas wymagany na sortowanie.

3. Algorytm First Fit sprawuje się znacznie lepiej od algorytmu Next Fit, kosztem znacznie zwiększonego narzutu czasowego (O(n) vs O(n^2)). Wydajność można polepszyć i zredukować złożoność obliczeniową do O(n log n) przez implementację w oparciu o drzewo binarne.

4. Dystrybucja rozmiarów elementów oraz ich kolejność znacząco wpływa na możliwość znalezienia optymalnego rozwiązania przez heurystyki Next Fit i First Fit, w szczególności Next Fit.

5. Algorytm First Fit dla mojej metody generacji danych był w stanie znaleźć optymalne rozwiązanie dla niektórych przypadków.

==== Możliwe ścieżki dalszego rozwoju projektu ====
  * Metaheurystyka
  * Inne sposoby generowania danych (generowanie optymalnego rozwiązania poprzez generowanie N losowych podziałów w kontenerze, zamiast obecnego dopełniania do wygenerowanych elementów)

==== Link do repozytorium projektu ====
  * [[https://github.com/asdfMaciej/bin-packing]] - kod wykonany w języku Rust
  * [[https://gist.github.com/asdfMaciej/ef8053d578768e60d3e1baed60051262]] - wzorcowe dane wejściowe [JSON]
  * [[https://gist.github.com/asdfMaciej/872266fefc698da1c49383fa91873f66]] - wzorcowe dane wyjściowe [JSON]
  * [[https://gist.github.com/asdfMaciej/7599e3091f355d4cc8370aa858c79cbf]] - wzorcowe dane wyjściowe [CSV]
 

