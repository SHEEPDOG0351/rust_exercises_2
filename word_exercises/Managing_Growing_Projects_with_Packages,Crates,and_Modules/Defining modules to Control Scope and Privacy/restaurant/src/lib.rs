mod front_of_house { // parent module, front_of_house. It's submodules represent the 'front of house' of a restaurant
    mod hosting { // sibilings
        fn add_to_waitlist() {} // children of 'hosting'

        fn seat_at_table() {} // children of 'hosting'
    }

    mod serving { // sibilings
        fn take_order() {} // children of 'serving'

        fn serve_order() {} // children of 'serving'

        fn take_payment() {} // children of 'serving'
    }
}