-- Your SQL goes here
INSERT INTO organization
    VALUES ('51897', 'TUM School of Computation, Information and Technology', '1', 'parent'),
    ('51267', 'TUM School of Engineering and Design', '1', 'school'),
    ('51898', 'TUM School of Natural Sciences', '1', 'school'),
    ('51258', 'TUM School of Life Sciences', '1', 'school'),
    ('51899', 'TUM School of Medicine and Health', '1', 'school'),
    ('51900', 'TUM School of Management', '1', 'school'),
    ('51901', 'TUM School of Social Sciences and Technology', '1', 'school'),
    ('53597', 'Department of Mathematics', '51897', 'department'),
    ('53598', 'Department of Computer Science', '51897', 'department'),
    ('53599', 'Department of Computer Engineering', '51897', 'department'),
    ('53600', 'Department of Electrical Engineering', '51897', 'department'),
    ('51837', 'Department of Aerospace and Geodesy', '51267', 'department'),
    ('51838', 'Department of Architecture', '51267', 'department'),
    ('51839', 'Department of Civil and Environmental Engineering', '51267', 'department'),
    ('51844', 'Department of Energy and Process Engineering', '51267', 'department'),
    ('51840', 'Department of Engineering Physics and Computation', '51267', 'department'),
    ('51843', 'Department of Materials Engineering', '51267', 'department'),
    ('51842', 'Department of Mechanical Engineering', '51267', 'department'),
    ('51841', 'Department of Mobility Systems Engineering', '51267', 'department'),
    ('53217', 'Department of Physics', '51898', 'department'),
    ('53218', 'Department of Bioscience', '51898', 'department'),
    ('53219', 'Department of Chemistry', '51898', 'department'),
    ('51264', 'Department of Molecular Life Sciences', '51258', 'department'),
    ('51265', 'Department of Life Science Engineering', '51258', 'department'),
    ('51266', 'Department of Life Science Systems', '51258', 'department'),
    ('54897', 'Department of Health and Sport Sciences', '51899', 'department'),
    ('54898', 'Department of Preclinical Medicine', '51899', 'department'),
    ('55465', 'Department of Preclinical Medicine', '51899', 'department'),
    ('54899', 'Department of Clinical Medicine', '51899', 'department'),
    ('55467', 'Department of Clinical Medicine', '51899', 'department'),
    ('52424', 'Department of Economics and Policy', '51900', 'department'),
    ('52425', 'Department of Finance and Accounting', '51900', 'department'),
    ('52426', 'Department of Innovation and Entrepreneurship', '51900', 'department'),
    ('52427', 'Department of Marketing, Strategy and Leadership', '51900', 'department'),
    ('52428', 'Department of Operations and Technology', '51900', 'department'),
    ('52497', 'Department of Educational Sciences', '51901', 'department'),
    ('52498', 'Department of Governance', '51901', 'department'),
    ('52517', 'Department of Science, Technology and Society', '51901', 'department'),
    ('15741', 'Institut für Allgemeine Pathologie und Pathologische Anatomie (Dr. Mogler komm.)', '54898', 'institute'),
    ('15744', 'Institut für Geschichte und Ethik der Medizin (Prof. Buyx)', '54898', 'institute'),
    ('15740', 'Institut für Medizinische Mikrobiologie, Immunologie und Hygiene (Prof. Busch)', '54898', 'institute'),
    ('27382', 'Institut für  Neurowissenschaften (Prof. Konnerth)', '54898', 'institute'),
    ('15742', 'Institut für Pharmakologie und Toxikologie (Prof. Engelhardt)', '54898', 'institute'),
    ('15747', 'Institut für Toxikologie und Umwelthygiene (Prof. Göttlicher)', '54898', 'institute'),
    ('15748', 'Institut für Virologie (Prof. Protzer)', '54898', 'institute'),
    ('46031', 'Institut für Zellbiologie des Nervensystems (Prof. Misgeld)', '54898', 'institute'),
    ('42655', 'Institut für  Allgemeinmedizin und Versorgungsforschung (Prof. Schneider)', '54899', 'institute'),
    ('15736', 'Institut für Diagnostische und Interventionelle Radiologie (Prof. Makowski)', '54899', 'institute'),
    ('54559', 'Institut für Experimentelle Hämatologie (Prof. Schmidt-Supprian)', '54899', 'institute'),
    ('54560', 'Institut für Experimentelle Neuroimmunologie (Prof. Korn)', '54899', 'institute'),
    ('54562', 'Institut für Experimentelle Tumortherapie (Prof. Saur)', '54899', 'institute'),
    ('15737', 'Institut für Humangenetik (Prof. Winkelmann)', '54899', 'institute'),
    ('15743', 'Institut für KI und Informatik in der Medizin (Prof. Rückert)', '54899', 'institute'),
    ('15738', 'Institut für Klinische Chemie und Pathobiochemie (Prof. Ruland)', '54899', 'institute'),
    ('45772', 'Institut für Molekulare Allergologie (Prof. Schmidt-Weber)', '54899', 'institute'),
    ('45774', 'Institut für Molekulare Immunologie (Prof. Knolle)', '54899', 'institute'),
    ('49897', 'Institut für  Molekulare Onkologie und Funktionelle Genomik (Prof. Rad)', '54899', 'institute')
ON CONFLICT (id)
    DO NOTHING;

-- Language centers
INSERT INTO organization
    VALUES ('26608', 'TUM Sprachenzentrum', '1', 'parent'),
    ('49037', 'Arabic', '26608', 'language center'),
    ('49038', 'Chinese', '26608', 'language center'),
    ('49039', 'Danish', '26608', 'language center'),
    ('49040', 'German as Foreign Language', '26608', 'language center'),
    ('49041', 'English', '26608', 'language center'),
    ('49042', 'French', '26608', 'language center'),
    ('52657', 'Sign Language', '26608', 'language center'),
    ('49043', 'Hebrew', '26608', 'language center'),
    ('49057', 'Intercultural Communication', '26608', 'language center'),
    ('49044', 'Italian', '26608', 'language center'),
    ('49045', 'Japanese', '26608', 'language center'),
    ('52457', 'Catalan', '26608', 'language center'),
    ('49046', 'Korean', '26608', 'language center'),
    ('49047', 'Dutch', '26608', 'language center'),
    ('49048', 'Norwegian', '26608', 'language center'),
    ('49049', 'Portuguese', '26608', 'language center'),
    ('49050', 'Russian', '26608', 'language center'),
    ('49051', 'Swedish', '26608', 'language center'),
    ('49052', 'Spanish', '26608', 'language center'),
    ('49053', 'Turkish', '26608', 'language center')
ON CONFLICT (id)
    DO NOTHING;

-- Medical Chairs
INSERT INTO organization
    VALUES ('54668', 'Diagnostische und Interventionelle Neuroradiologie (Prof. Zimmer)', '54899', 'chair')
ON CONFLICT (id)
    DO NOTHING;

-- Brewing
INSERT INTO organization
    VALUES ('51745', 'Academic Programs Brewing, Food Technology and Bioprocess Engeneering', '51258', 'academic programs'),
    ('51937', 'Academic Program Administration of the TUM School of Engineering and Design', '51267', 'academic programs'),
    ('52301', 'Academic Programs Context teaching', '51901', 'academic programs'),
    ('51751', 'Academic Programs Brewing', '51745', 'academic program'),
    ('51752', 'Academic Programs Food Technology and Bioprocess Engineering', '51745', 'academic program'),
    ('52099', 'Academic Programs Study Program Brewing, Food Technology and Bioprocess Engeneering', '51745', 'academic program')
ON CONFLICT (id)
    DO NOTHING;

INSERT INTO organization
    VALUES ('45350', 'Integrative Research Institutes', '1', 'parent'),
    ('49102', 'TUM Campus Straubing für Biotechnologie und Nachhaltigkeit (TUMCS)', '45350', 'integrative research institute'),
    ('16301', 'Institutions close to the University', '1', 'parent')
ON CONFLICT (id)
    DO NOTHING;

