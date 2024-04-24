/// Структура данных KeyValuePair представляет пару "ключ-значение".
struct KeyValuePair<K, V> {
    key: K,
    value: V,
}

impl<K, V> KeyValuePair<K, V> {
    /// Создает новую пару "ключ-значение".
    fn new(key: K, value: V) -> Self {
        KeyValuePair { key, value }
    }
}

/// Структура данных Map представляет словарь с парами "ключ-значение".
struct Map<K, V> {
    items: Vec<KeyValuePair<K, V>>,
}

impl<K, V> Map<K, V>
where
    K: PartialEq, 
{
    /// Создает новый пустой словарь.
    fn new() -> Self {
        Map { items: Vec::new() }
    }

    /// Вставляет новую пару "ключ-значение" в словарь.
    fn insert(&mut self, key: K, value: V) {
        self.items.push(KeyValuePair::new(key, value));
    }

    /// Получает значение по ключу из словаря.
    ///
    /// # Returns
    ///
    /// `Some(&V)` - значение, если ключ найден в словаре.
    /// `None` - если ключ не найден в словаре.
    fn get(&self, key: &K) -> Option<&V> {
        for item in &self.items {
            if &item.key == key {
                return Some(&item.value);
            }
        }
        None
    }

    /// Удаляет пару "ключ-значение" из словаря по ключу.
    fn remove(&mut self, key: &K) {
        self.items.retain(|item| &item.key != key);
    }

    /// Проверяет наличие ключа в словаре.
    ///
    /// # Returns
    ///
    /// `true` - если ключ найден в словаре.
    /// `false` - если ключ не найден в словаре.
    fn contains_key(&self, key: &K) -> bool {
        self.items.iter().any(|item| &item.key == key)
    }

    /// Возвращает количество элементов в словаре.
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Проверяет, является ли словарь пустым.
    ///
    /// # Returns
    ///
    /// `true` - если словарь пуст.
    /// `false` - если словарь не пуст.
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn main() {
    // Создаем новую карту
    let mut map = Map::new();

    // Вставляем элементы в карту
    map.insert("apple", 2);
    map.insert("banana", 3);
    map.insert("orange", 5);

    // Проверяем методы с помощью assert

    // Проверяем метод get
    assert_eq!(map.get(&"apple"), Some(&2));
    assert_eq!(map.get(&"banana"), Some(&3));
    assert_eq!(map.get(&"orange"), Some(&5));

    // Проверяем метод contains_key
    assert_eq!(map.contains_key(&"apple"), true);
    assert_eq!(map.contains_key(&"banana"), true);
    assert_eq!(map.contains_key(&"orange"), true);
    assert_eq!(map.contains_key(&"grape"), false);

    // Проверяем метод len
    assert_eq!(map.len(), 3);

    // Удаляем элемент из карты
    map.remove(&"banana");

    // Проверяем, что элемент был удален
    assert_eq!(map.contains_key(&"banana"), false);

    // Проверяем метод is_empty
    assert_eq!(map.is_empty(), false);

    // Удаляем оставшиеся элементы из карты
    map.remove(&"apple");
    map.remove(&"orange");

    // Проверяем, что карта теперь пуста
    assert_eq!(map.is_empty(), true);
}
