use lexicon_stripper::{Domain, LexiconStripper, StripperConfig};

#[test]
fn test_culinary_domain_detection_overrides_spurious_finance() {
    let stripper = LexiconStripper::new();
    let text = "The fine dining industry has astronomical profit margins on tap water. \
                The head chef relies on an army of unpaid stagiaires and works with produce \
                to keep the Michelin inspectors entertained before investors realize the business model.";

    let config = StripperConfig {
        domain: Domain::Auto,
        seed: Some(42),
        ..Default::default()
    };

    let result = stripper.transform(text, &config);
    // Culinary domain should be detected despite "profit margins" and "investors"
    assert_eq!(
        result.report.detected_domain,
        "culinary",
        "Expected culinary domain to be detected over spurious finance terms, got: {}",
        result.report.detected_domain
    );
}

#[test]
fn test_hot_pan_never_becomes_freshly_brewed_pan() {
    let stripper = LexiconStripper::new();
    let text = "The head chef was furious and threatened to throw a hot pan at the assistant.";

    // Try multiple seeds to thoroughly check candidate selection
    for seed in 1..=20 {
        let config = StripperConfig {
            domain: Domain::Culinary,
            seed: Some(seed),
            replacement_probability: 1.0,
            ..Default::default()
        };

        let result = stripper.transform(text, &config);
        assert!(
            !result.transformed_text.contains("freshly brewed pan"),
            "Seed {} produced unnatural 'freshly brewed pan': {}",
            seed,
            result.transformed_text
        );
        assert!(
            !result.transformed_text.contains("freshly made pan"),
            "Seed {} produced unnatural 'freshly made pan': {}",
            seed,
            result.transformed_text
        );
    }
}

#[test]
fn test_keep_entertained_valency_preservation() {
    let stripper = LexiconStripper::new();
    let text = "We serve foams and gels just to keep the Michelin inspectors sufficiently entertained.";

    for seed in 1..=20 {
        let config = StripperConfig {
            domain: Domain::Culinary,
            seed: Some(seed),
            replacement_probability: 1.0,
            ..Default::default()
        };

        let result = stripper.transform(text, &config);
        assert!(
            !result.transformed_text.contains("sustain the Michelin inspectors"),
            "Seed {} produced ungrammatical 'sustain the Michelin inspectors': {}",
            seed,
            result.transformed_text
        );
        assert!(
            !result.transformed_text.contains("preserve the Michelin inspectors"),
            "Seed {} produced ungrammatical 'preserve the Michelin inspectors': {}",
            seed,
            result.transformed_text
        );
    }
}

#[test]
fn test_all_expanded_domains_detectable() {
    let stripper = LexiconStripper::new();

    let domain_samples = [
        ("culinary", "The executive chef prepared a tasting menu with fresh produce, turnip puree, and fine dining wine."),
        ("creative_arts", "The museum curator organized an exhibition featuring a Renaissance triptych, chiaroscuro paintings, and modern sculpture."),
        ("science", "The physics laboratory conducted spectroscopy experiments measuring entropy, quantum mechanics, and catalyst reactions."),
        ("politics", "The parliamentary election saw heated debate over foreign policy, bipartisan legislation, and the controversial referendum."),
        ("sports", "The championship tournament concluded with an incredible performance by the athlete, goalkeeper, and decathlon team."),
        ("education", "The university provost revised the curriculum, faculty pedagogy, syllabus standards, and student matriculation requirements."),
        ("philosophy_psychology", "The paper explored epistemology, cognitive dissonance, phenomenology, and the nature of subconscious perception."),
        ("military", "The battalion commander deployed reconnaissance units, infantry brigades, and artillery support to secure the flank."),
        ("journalism_media", "The newspaper published an investigative journalism piece with a front-page byline, editorial op-ed, and press release."),
        ("travel_tourism", "The travel itinerary guided tourists to the resort destination with concierge services, excursion passes, and sightseeing tours."),
        ("aviation_aerospace", "The avionics technician monitored telemetry, altimeter readings, and fuselage stress during high mach flight dynamics."),
        ("architecture_construction", "The cantilever structure featured brutalism design, load-bearing walls, masonry facade, and modern HVAC planning."),
        ("gaming_esports", "The competitive esports tournament analyzed player hitbox data, aggro mechanics, and frame data in matchmaking."),
        ("agriculture_botany", "The crop science facility used hydroponics, plant grafting, agronomy techniques, and cultivar photosynthesis tracking."),
        ("fashion_textiles", "The haute couture collection showcased bias cut silk, selvage denim, elegant silhouette draping, and bespoke tailoring."),
        ("theology_religion", "The seminary professor conducted scripture exegesis on ecclesiastical dogma, ancient liturgy, and orthodox theology."),
        ("audio_engineering", "The recording studio applied reverb, equalization, lossless compression, and acoustic attenuation to the sound design."),
        ("maritime_nautical", "The vessel steered to starboard using dead reckoning while the crew checked the ballast, keel, and halyard."),
        ("automotive_motorsport", "The racing chassis suffered excessive oversteer when the camshaft and drivetrain telemetry reached peak torque."),
        ("film_television", "The director oversaw the mise-en-scène and color grading, using anamorphic lenses and precise foley on the soundstage."),
        ("linguistics_philology", "The phonology paper analyzed the phoneme distribution, morphology syntax, fricative consonants, and diphthong shifts."),
        ("real_estate_property", "The real estate closing costs were held in escrow after the appraisal and title search identified a municipal lien."),
        ("logistics_supply_chain", "The intermodal freight shipment experienced supply chain bottleneck delays at the regional distribution center."),
        ("fitness_kinesiology", "The athlete trained for muscle hypertrophy and strength with deadlift superset routines and isometric barbell exercises."),
        ("occult_astrology", "The astrologer prepared a natal chart and studied planetary retrograde movements alongside esoteric tarot divination."),
    ];

    for (expected_domain, sample_text) in domain_samples {
        let config = StripperConfig {
            domain: Domain::Auto,
            seed: Some(100),
            ..Default::default()
        };

        let result = stripper.transform(sample_text, &config);
        assert_eq!(
            result.report.detected_domain,
            expected_domain,
            "Failed for text '{}': expected {}, got {}",
            sample_text,
            expected_domain,
            result.report.detected_domain
        );
    }
}
