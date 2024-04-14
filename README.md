# BambangShop Publisher App

Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project

In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:

1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment

1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable | type | description |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)

-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections

This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1.  ##### Observer Pattern
    > In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

    _Observer design pattern_ memberikan solusi standar untuk problem umum seperti sistem notifikasi atau sistem lainnya yang melibatkan adanya observer/subscriber dan subject/publisher. Berikut merupakan definisi berdasarkan buku Head First Design Pattern book.

    ![Observer Pattern Definition - Diagram](/assets/images/observer-pattern-definition.png)

    Pada diagram tersebut, masing-masing role mempunyai interface sendiri. Interface `Subject` memiliki method untuk menambahkan, menghapus, dan memberikan notifikasi kepada Observer. Adapun, interface `Observer` memiliki method update yang akan dipanggil oleh objek Subject ketika memberikan notifikasi. 
    
    Pada case aplikasi BambangShop ini, kita juga menerapkan Observer Pattern. Pada kasus ini, terminologi yang dipakai adalah Subscriber & Publisher. _Struct_ `Subscriber` didefinisikan sebagai berikut.

    ```rs
    pub struct Subscriber {
        pub url: String,
        pub name: String,
    }
    ```

    Pada kasus ini, entitas yang mengikuti Observer pattern dipisahkan oleh aplikasi yang berbeda. Aplikasi ini berperan sebagai Publisher. Untuk memberikan notifikasi kepada Subscriber di aplikasi lain, kita dapat berkomunikasi melalui _network protocol_ seperti HTTP. Untuk itu, kita perlu menyimpan URL dari aplikasi subscribers di aplikasi publisher. Komunikasi melalui URL juga dapat mengganti pemanggilan method `update` pada diagram di atas. Hal ini karena keduanya sama sama bertujuan untuk memberikan notifikasi kepada Subscriber. Jadi, interface/trait untuk struct Subscriber tidak diperlukan.

2. ##### Vec (list) or DashMap (Map)?
    > id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?

    Pada implementasi `ProductRepository` dan `SubscriberRepository`, kita menggunakan DashMap untuk menyimpan data. Pada `ProductRepository`, key-nya adalah id<usize>. Sedangkan pada `SubscriberRepository`, terdapat dua level Dashmap. Outer level-nya menggunakan _product\_type_ sebagai key dan inner level-nya menggunakan _url_ sebagai key. 
    
    Tentunya implementasi DashMap pada `SubscriberRepository` sangat diperlukan karena struktur penyimpanan datanya yang cukup kompleks. Adapun pada `ProductRepository`, implementansi DashMap dapat digantikan oleh Vec. Namun tentunya, terdapat _cost performance_ yang lebih besar ketika melakukan operasi pada elemen spesifik tertentu.

3. ##### DashMap or Singleton Pattern ?
    > When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?

    _Singleton pattern_ dan penggunaan DashMap dalam proyek ini memiliki tujuan yang berbeda yang tidak _mutually exclusive_.

    _Singleton pattern_ memastikan bahwa suatu class hanya dapat diinstansiasi sebanyak satu kali untuk kemudian dapat digunakan dalam berbagai penggunaan. Pada potongan kode ini, _singleton pattern_ telah diimplementasikan dengan menggunakan _macro_ `lazy_static` pada variabel `SUBSCRIBERS`. Macro ini memastikan hanya ada satu _instance_ `SUBSCRIBERS` dalam seluruh sistem aplikasi.

    Sedangkan, DashMap merupakan implementasi HashMap dengan concurrency di dalam Rust. DashMap memungkinkan akses dan update data pada variabel `SUBSCRIBERS` secara concurrent dengan aman. Jika tidak menggunakan DashMap atau struktur data concurrent lainnya, dikhawatirkan terjadi _data races_ atau _undefined behaviour_.

    Dengan kata lain, keduanya dibutuhkan dalam konteks ini.

#### Reflection Publisher-2

#### Reflection Publisher-3
