use rand::prelude::*;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Book {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    Samuel1,
    Samuel2,
    Kings1,
    Kings2,
    Chronicles1,
    Chronicles2,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    Corinthians1,
    Corinthians2,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    Thessalonians1,
    Thessalonians2,
    Timothy1,
    Timothy2,
    Titus,
    Philemon,
    Hebrews,
    James,
    Peter1,
    Peter2,
    John1,
    John2,
    John3,
    Jude,
    Revelation,
}

impl Book {
    pub fn to_string(&self) -> String {
        match self {
            Book::Acts => "Acts",
            Book::Amos => "Amos",
            Book::Chronicles1 => "1 Chronicles",
            Book::Chronicles2 => "2 Chronicles",
            Book::Colossians => "Colossians",
            Book::Corinthians1 => "1 Corinthians",
            Book::Corinthians2 => "2 Corinthians",
            Book::Daniel => "Daniel",
            Book::Deuteronomy => "Deuteronomy",
            Book::Ecclesiastes => "Ecclesiastes",
            Book::Ephesians => "Ephesians",
            Book::Esther => "Esther",
            Book::Exodus => "Exodus",
            Book::Ezekiel => "Ezekiel",
            Book::Ezra => "Ezra",
            Book::Galatians => "Galatians",
            Book::Genesis => "Genesis",
            Book::Habakkuk => "Habakkuk",
            Book::Haggai => "Haggai",
            Book::Hebrews => "Hebrews",
            Book::Hosea => "Hosea",
            Book::Isaiah => "Isaiah",
            Book::James => "James",
            Book::Jeremiah => "Jeremiah",
            Book::Job => "Job",
            Book::Joel => "Joel",
            Book::John => "John",
            Book::John1 => "1 John",
            Book::John2 => "2 John",
            Book::John3 => "3 John",
            Book::Jonah => "Jonah",
            Book::Joshua => "Joshua",
            Book::Jude => "Jude",
            Book::Judges => "Judges",
            Book::Kings1 => "1 Kings",
            Book::Kings2 => "2 Kings",
            Book::Lamentations => "Lamentations",
            Book::Leviticus => "Leviticus",
            Book::Luke => "Luke",
            Book::Malachi => "Malachi",
            Book::Mark => "Mark",
            Book::Matthew => "Matthew",
            Book::Micah => "Micah",
            Book::Nahum => "Nahum",
            Book::Nehemiah => "Nehemiah",
            Book::Numbers => "Numbers",
            Book::Obadiah => "Obadiah",
            Book::Peter1 => "1 Peter",
            Book::Peter2 => "2 Peter",
            Book::Philemon => "Philemon",
            Book::Philippians => "Philippians",
            Book::Proverbs => "Proverbs",
            Book::Psalms => "Psalms",
            Book::Revelation => "Revelation",
            Book::Romans => "Romans",
            Book::Ruth => "Ruth",
            Book::Samuel1 => "1 Samuel",
            Book::Samuel2 => "2 Samuel",
            Book::SongOfSolomon => "Song of Solomon",
            Book::Thessalonians1 => "1 Thessalonians",
            Book::Thessalonians2 => "2 Thessalonians",
            Book::Timothy1 => "1 Timothy",
            Book::Timothy2 => "2 Timothy",
            Book::Titus => "Titus",
            Book::Zechariah => "Zechariah",
            Book::Zephaniah => "Zephaniah",
        }.to_string()
    }
}

#[derive(Clone)]
pub struct Reference {
    pub book: Book,
    pub chapter: u8,
    pub start_verse: u8,
    pub end_verse: u8,
}

impl Reference {
    pub fn to_string(&self) -> String {
        let mut result = format!("{} {}:{}", self.book.to_string(), self.chapter, self.start_verse);
        if self.end_verse > self.start_verse {
            result += &format!("-{}", self.end_verse);
        }
        result
    }
}

#[derive(Clone)]
pub struct Verse {
    pub reference: Reference,
    pub text: &'static str,
}

const VERSES: &'static [Verse; 172] = &[
        Verse {
            reference: Reference {
                book: Book::Exodus,
                chapter: 19,
                start_verse: 4,
                end_verse: 6,
            },
            text: "You yourselves have seen what I did to the Egyptians, and how I bore you on eagles' wings and brought you to myself.  Now therefore, if you will indeed obey my voice and keep my covenant, you shall be my treasured possession among all peoples, for all the earth is mine; and you shall be to me a kingdom of priests and a holy nation.",
        }, Verse {
            reference: Reference {
                book: Book::Deuteronomy,
                chapter: 6,
                start_verse: 4,
                end_verse: 5,
            },
            text: "Hear, O Israel:  The LORD our God, the LORD is one.  You shall love the LORD your God with all your heart and with all your soul and with all your might.",
        }, Verse {
            reference: Reference {
                book: Book::Joshua,
                chapter: 1,
                start_verse: 7,
                end_verse: 9,
            },
            text: "Only be strong and very courageous, being careful to do according to all the law that Moses my servant commanded you.  Do not turn from it to the right hand or to the left, that you may have good success wherever you go.  This Book of the Law shall not depart from your mouth, but you shall meditate on it day and night, so that you may be careful to do according to all that is written in it.  For then you will make your way prosperous, and then you will have good success.  Have I not commanded you?  Be strong and courageous.  Do not be frightened, and do not be dismayed, for the LORD your God is with you wherever you go.",
        }, Verse {
            reference: Reference {
                book: Book::Samuel1,
                chapter: 12,
                start_verse: 23,
                end_verse: 23,
            },
            text: "Moreover, as for me, far be it from me that I should sin against the LORD by ceasing to pray for you, and I will instruct you in the good and the right way.",
        }, Verse {
            reference: Reference {
                book: Book::Samuel1,
                chapter: 15,
                start_verse: 22,
                end_verse: 22,
            },
            text: "Has the LORD as great delight in burnt offerings and sacrifices, as in obeying the voice of the LORD?  Behold, to obey is better than sacrifice, and to listen than the fat of rams.",
        }, Verse {
            reference: Reference {
                book: Book::Chronicles2,
                chapter: 7,
                start_verse: 14,
                end_verse: 14,
            },
            text: "If my people who are called by my name humble themselves, and pray and seek my face and turn from their wicked ways, then I will hear from heaven and will forgive their sin and heal their land.",
        }, Verse {
            reference: Reference {
                book: Book::Chronicles2,
                chapter: 16,
                start_verse: 9,
                end_verse: 9,
            },
            text: "For the eyes of the LORD run to and fro throughout the whole earth, to give strong support to those whose heart is blameless toward him.",
        }, Verse {
            reference: Reference {
                book: Book::Job,
                chapter: 19,
                start_verse: 25,
                end_verse: 27,
            },
            text: "For I know that my Redeemer lives, and at the last he will stand upon the earth.  And after my skin has been thus destroyed, yet in my flesh I shall see God, whom I shall see for myself, and my eyes shall behold, and not another.  My heart faints within me!",
        }, Verse {
            reference: Reference {
                book: Book::Job,
                chapter: 42,
                start_verse: 5,
                end_verse: 6,
            },
            text: "I had heard of you by the hearing of the ear, but now my eye sees you; therefore I despise myself, and repent in dust and ashes.",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 16,
                start_verse: 2,
                end_verse: 3,
            },
            text: "I said to the LORD, “You are my Lord; I have no good apart from you.”  As for the saints in the land, they are the excellent ones, in whom is all my delight.",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 34,
                start_verse: 8,
                end_verse: 9,
            },
            text: "Oh, taste and see that the LORD is good!  Blessed is the man who takes refuge in him!  Oh, fear the LORD, you his saints, for those who fear him have no lack!",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 37,
                start_verse: 4,
                end_verse: 7,
            },
            text: "Delight yourself in the LORD, and he will give you the desires of your heart.  Commit your way to the LORD; trust in him, and he will act.  He will bring forth your righteousness as the light, and your justice as the noonday.  Be still before the LORD and wait patiently for him; fret not yourself over the one who prospers in his way, over the man who carries out evil devices!",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 46,
                start_verse: 10,
                end_verse: 10,
            },
            text: "Be still, and know that I am God.  I will be exalted among the nations, I will be exalted in the earth!",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 51,
                start_verse: 10,
                end_verse: 12,
            },
            text: "Create in me a clean heart, O God, and renew a right spirit within me.  Cast me not away from your presence, and take not your Holy Spirit from me.  Restore to me the joy of your salvation, and uphold me with a willing spirit.",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 84,
                start_verse: 10,
                end_verse: 10,
            },
            text: "For a day in your courts is better than a thousand elsewhere.  I would rather be a doorkeeper in the house of my God than dwell in the tents of wickedness.",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 103,
                start_verse: 8,
                end_verse: 12,
            },
            text: "The LORD is merciful and gracious, slow to anger and abounding in steadfast love.  He will not always chide, nor will he keep his anger forever.  He does not deal with us according to our sins, nor repay us according to our iniquities.  For as high as the heavens are above the earth, so great is his steadfast love toward those who fear him; as far as the east is from the west, so far does he remove our transgressions from us.",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 119,
                start_verse: 11,
                end_verse: 11,
            },
            text: "I have stored up your word in my heart, that I might not sin against you.",
        }, Verse {
            reference: Reference {
                book: Book::Psalms,
                chapter: 119,
                start_verse: 105,
                end_verse: 105,
            },
            text: "Your word is a lamp to my feet and a light to my path.",
        }, Verse {
            reference: Reference {
                book: Book::Proverbs,
                chapter: 3,
                start_verse: 5,
                end_verse: 6,
            },
            text: "Trust in the LORD with all your heart, and do not lean on your own understanding.  In all your ways acknowledge him, and he will make straight your paths.",
        }, Verse {
            reference: Reference {
                book: Book::Isaiah,
                chapter: 1,
                start_verse: 18,
                end_verse: 18,
            },
            text: "Come now, let us reason together, says the LORD:  though your sins are like scarlet, they shall be as white as snow; though they are red like crimson, they shall become like wool.",
        }, Verse {
            reference: Reference {
                book: Book::Isaiah,
                chapter: 40,
                start_verse: 6,
                end_verse: 8,
            },
            text: "A voice says, “Cry!”  And I said, “What shall I cry?”  All flesh is grass, and all its beauty is like the flower of the field.  The grass withers, the flower fades when the breath of the LORD blows on it; surely the people are grass.  The grass withers, the flower fades, but the word of our God will stand forever.",
        }, Verse {
            reference: Reference {
                book: Book::Isaiah,
                chapter: 40,
                start_verse: 28,
                end_verse: 31,
            },
            text: "Have you not known?  Have you not heard?  The LORD is the everlasting God, the Creator of the ends of the earth.  He does not faint or grow weary; his understanding is unsearchable.  He gives power to the faint, and to him who has no might he increases strength.  Even youths shall faint and be weary, and young men shall fall exhausted; but they who wait for the LORD shall renew their strength; they shall mount up with wings like eagles; they shall run and not be weary; they shall walk and not faint.",
        }, Verse {
            reference: Reference {
                book: Book::Isaiah,
                chapter: 54,
                start_verse: 2,
                end_verse: 3,
            },
            text: "Enlarge the place of your tent, and let the curtains of your habitations be stretched out; do not hold back; lengthen your cords and strengthen your stakes.  For you will spread abroad to the right and to the left, and your offspring will possess the nations and will people the desolate cities.",
        }, Verse {
            reference: Reference {
                book: Book::Isaiah,
                chapter: 55,
                start_verse: 1,
                end_verse: 3,
            },
            text: "Come, everyone who thirsts, come to the waters; and he who has no money, come, buy and eat!  Come, buy wine and milk without money and without price.  Why do you spend your money for that which is not bread, and your labor for that which does not satisfy?  Listen diligently to me, and eat what is good, and delight yourselves in rich food.  Incline your ear, and come to me; hear, that your soul may live.",
        }, Verse {
            reference: Reference {
                book: Book::Isaiah,
                chapter: 55,
                start_verse: 8,
                end_verse: 9,
            },
            text: "For my thoughts are not your thoughts, neither are your ways my ways, declares the LORD.  For as the heavens are higher than the earth, so are my ways higher than your ways and my thoughts than your thoughts.",
        }, Verse {
            reference: Reference {
                book: Book::Isaiah,
                chapter: 57,
                start_verse: 15,
                end_verse: 15,
            },
            text: "For thus says the One who is high and lifted up, who inhabits eternity, whose name is Holy: “I dwell in the high and holy place, and also with him who is of a contrite and lowly spirit, to revive the spirit of the lowly, and to revive the heart of the contrite.”",
        }, Verse {
            reference: Reference {
                book: Book::Isaiah,
                chapter: 66,
                start_verse: 1,
                end_verse: 2,
            },
            text: "Thus says the LORD:  “Heaven is my throne, and the earth is my footstool; what is the house that you would build for me, and what is the place of my rest?  All these things my hand has made, and so all these things came to be, declares the LORD.  But this is the one to whom I will look:  he who is humble and contrite in spirit and trembles at my word.”",
        }, Verse {
            reference: Reference {
                book: Book::Jeremiah,
                chapter: 9,
                start_verse: 23,
                end_verse: 24,
            },
            text: "Thus says the LORD:  “Let not the wise man boast in his wisdom, let not the mighty man boast in his might, let not the rich man boast in his riches, but let him who boasts boast in this, that he understands and knows me, that I am the LORD who practices steadfast love, justice, and righteousness in the earth.  For in these things I delight, declares the LORD.”",
        }, Verse {
            reference: Reference {
                book: Book::Jeremiah,
                chapter: 29,
                start_verse: 11,
                end_verse: 13,
            },
            text: "For I know the plans I have for you, declares the LORD, plans for welfare and not for evil, to give you a future and a hope.  Then you will call upon me and come and pray to me, and I will hear you.  You will seek me and find me, when you seek me with all your heart.",
        }, Verse {
            reference: Reference {
                book: Book::Jeremiah,
                chapter: 33,
                start_verse: 3,
                end_verse: 3,
            },
            text: "Call to me and I will answer you, and will tell you great and hidden things that you have not known.",
        }, Verse {
            reference: Reference {
                book: Book::Lamentations,
                chapter: 3,
                start_verse: 21,
                end_verse: 23,
            },
            text: "But this I call to mind, and therefore I have hope:  The steadfast love of the LORD never ceases; his mercies never come to an end; they are new every morning; great is your faithfulness.",
        }, Verse {
            reference: Reference {
                book: Book::Joel,
                chapter: 2,
                start_verse: 13,
                end_verse: 13,
            },
            text: "“[…] and rend your hearts and not your garments.”  Return to the LORD your God, for he is gracious and merciful, slow to anger, and abounding in steadfast love; and he relents over disaster.",
        }, Verse {
            reference: Reference {
                book: Book::Micah,
                chapter: 6,
                start_verse: 8,
                end_verse: 8,
            },
            text: "He has told you, O man, what is good; and what does the LORD require of you but to do justice, and to love kindness, and to walk humbly with your God?",
        }, Verse {
            reference: Reference {
                book: Book::Habakkuk,
                chapter: 3,
                start_verse: 17,
                end_verse: 18,
            },
            text: "Though the fig tree should not blossom, nor fruit be on the vines, the produce of the olive fail and the fields yield no food, the flock be cut off from the fold and there be no herd in the stalls, yet I will rejoice in the LORD; I will take joy in the God of my salvation.",
        },
        Verse {
            reference: Reference {
                book: Book::Zephaniah,
                chapter: 3,
                start_verse: 17,
                end_verse: 17,
            },
            text: "The LORD your God is in your midst, a mighty one who will save; he will rejoice over you with gladness; he will quiet you by his love; he will exult over you with loud singing.",
        }, Verse {
            reference: Reference {
                book: Book::Zechariah,
                chapter: 4,
                start_verse: 6,
                end_verse: 6,
            },
            text: "Not by might, nor by power, but by my Spirit, says the LORD of hosts.",
        }, Verse {
            reference: Reference {
                book: Book::Malachi,
                chapter: 3,
                start_verse: 10,
                end_verse: 10,
            },
            text: "Bring the full tithe into the storehouse, that there may be food in my house.  And thereby put me to the test, says the LORD of hosts, if I will not open the windows of heaven for you and pour down for you a blessing until there is no more need.",
        }, Verse {
            reference: Reference {
                book: Book::Matthew,
                chapter: 6,
                start_verse: 19,
                end_verse: 21,
            },
            text: "Do not lay up for yourselves treasures on earth, where moth and rust destroy and where thieves break in and steal, but lay up for yourselves treasures in heaven, where neither moth nor rust destroys and where thieves do not break in and steal.  For where your treasure is, there your heart will be also.",
        }, 
        
        
        Verse {
            reference: Reference {
                book: Book::Matthew,
                chapter: 6,
                start_verse: 24,
                end_verse: 24,
            },
            text: "No one can serve two masters, for either he will hate the one and love the other, or he will be devoted to the one and despise the other.  You cannot serve God and money.",
        }, Verse {
            reference: Reference {
                book: Book::Matthew,
                chapter: 6,
                start_verse: 33,
                end_verse: 34,
            },
            text: "But seek first the kingdom of God and his righteousness, and all these things will be added to you.  Therefore do not be anxious about tomorrow, for tomorrow will be anxious for itself.  Sufficient for the day is its own trouble.",
        }, Verse {
            reference: Reference {
                book: Book::Matthew,
                chapter: 7,
                start_verse: 7,
                end_verse: 8,
            },
            text: "Ask, and it will be given to you; seek, and you will find; knock, and it will be opened to you.  For everyone who asks receives, and the one who seeks finds, and to the one who knocks it will be opened.",
        }, Verse {
            reference: Reference {
                book: Book::Matthew,
                chapter: 7,
                start_verse: 13,
                end_verse: 14,
            },
            text: "Enter by the narrow gate.  For the gate is wide and the way is easy that leads to destruction, and those who enter by it are many.  For the gate is narrow and the way is hard that leads to life, and those who find it are few.",
        }, Verse {
            reference: Reference {
                book: Book::Matthew,
                chapter: 11,
                start_verse: 28,
                end_verse: 29,
            },
            text: "Come to me, all who labor and are heavy laden, and I will give you rest.  Take my yoke upon you, and learn from me, for I am gentle and lowly in heart, and you will find rest for your souls.",
        }, Verse {
            reference: Reference {
                book: Book::Matthew,
                chapter: 22,
                start_verse: 37,
                end_verse: 39,
            },
            text: "And he said to him, “You shall love the Lord your God with all your heart and with all your soul and with all your mind.  This is the great and first commandment.  And a second is like it:  You shall love your neighbor as yourself.”",
        }, Verse {
            reference: Reference {
                book: Book::Matthew,
                chapter: 28,
                start_verse: 18,
                end_verse: 20,
            },
            text: "And Jesus came and said to them, “All authority in heaven and on earth has been given to me.  Go therefore and make disciples of all nations, baptizing them in the name of the Father and of the Son and of the Holy Spirit, teaching them to observe all that I have commanded you.  And behold, I am with you always, to the end of the age.”",
        }, Verse {
            reference: Reference {
                book: Book::Mark,
                chapter: 10,
                start_verse: 43,
                end_verse: 45,
            },
            text: "But it shall not be so among you.  But whoever would be great among you must be your servant, and whoever would be first among you must be slave of all.  For even the Son of Man came not to be served but to serve, and to give his life as a ransom for many.",
        }, Verse {
            reference: Reference {
                book: Book::Luke,
                chapter: 9,
                start_verse: 23,
                end_verse: 23,
            },
            text: "And he said to all, “If anyone would come after me, let him deny himself and take up his cross daily and follow me.”",
        }, Verse {
            reference: Reference {
                book: Book::Luke,
                chapter: 9,
                start_verse: 24,
                end_verse: 25,
            },
            text: "For whoever would save his life will lose it, but whoever loses his life for my sake will save it.  For what does it profit a man if he gains the whole world and loses or forfeits himself?",
        }, Verse {
            reference: Reference {
                book: Book::Luke,
                chapter: 10,
                start_verse: 20,
                end_verse: 20,
            },
            text: "Nevertheless, do not rejoice in this, that the spirits are subject to you, but rejoice that your names are written in heaven.",
        }, Verse {
            reference: Reference {
                book: Book::Luke,
                chapter: 11,
                start_verse: 13,
                end_verse: 13,
            },
            text: "If you then, who are evil, know how to give good gifts to your children, how much more will the heavenly Father give the Holy Spirit to those who ask him!",
        }, Verse {
            reference: Reference {
                book: Book::Luke,
                chapter: 12,
                start_verse: 32,
                end_verse: 32,
            },
            text: "Fear not, little flock, for it is your Father’s good pleasure to give you the kingdom.",
        }, Verse {
            reference: Reference {
                book: Book::Luke,
                chapter: 12,
                start_verse: 48,
                end_verse: 48,
            },
            text: "Everyone to whom much was given, of him much will be required; and from him to whom they entrusted much, they will demand the more.",
        }, Verse {
            reference: Reference {
                book: Book::Luke,
                chapter: 14,
                start_verse: 26,
                end_verse: 26,
            },
            text: "If anyone comes to me and does not hate his own father and mother and wife and children and brothers and sisters, yes, and even his own life, he cannot be my disciple.",
        }, Verse {
            reference: Reference {
                book: Book::Luke,
                chapter: 17,
                start_verse: 10,
                end_verse: 10,
            },
            text: "So you also, when you have done all that you were commanded, say, ‘We are unworthy servants; we have only done what was our duty.’",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 1,
                start_verse: 12,
                end_verse: 12,
            },
            text: "But to all who did receive him, who believed in his name, he gave the right to become children of God.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 1,
                start_verse: 14,
                end_verse: 14,
            },
            text: "And the Word became flesh and dwelt among us, and we have seen his glory, glory as of the only Son from the Father, full of grace and truth.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 3,
                start_verse: 16,
                end_verse: 16,
            },
            text: "For God so loved the world, that he gave his only Son, that whoever believes in him should not perish but have eternal life.  For God did not send his Son into the world to condemn the world, but in order that the world might be saved through him.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 5,
                start_verse: 44,
                end_verse: 44,
            },
            text: "How can you believe, when you receive glory from one another and do not seek the glory that comes from the only God?",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 6,
                start_verse: 68,
                end_verse: 69,
            },
            text: "Simon Peter answered him, “Lord, to whom shall we go?  You have the words of eternal life, and we have believed, and have come to know, that you are the Holy One of God.”",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 8,
                start_verse: 31,
                end_verse: 32,
            },
            text: "So Jesus said to the Jews who had believed him, “If you abide in my word, you are truly my disciples, and you will know the truth, and the truth will set you free.”",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 10,
                start_verse: 10,
                end_verse: 11,
            },
            text: "The thief comes only to steal and kill and destroy.  I came that they may have life and have it abundantly.  I am the good shepherd.  The good shepherd lays down his life for the sheep.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 10,
                start_verse: 27,
                end_verse: 30,
            },
            text: "My sheep hear my voice, and I know them, and they follow me.  I give them eternal life, and they will never perish, and no one will snatch them out of my hand.  My Father, who has given them to me, is greater than all, and no one is able to snatch them out of the Father's hand.  I and the Father are one.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 12,
                start_verse: 24,
                end_verse: 25,
            },
            text: "Truly, truly, I say to you, unless a grain of wheat falls into the earth and dies, it remains alone; but if it dies, it bears much fruit.  Whoever loves his life loses it, and whoever hates his life in this world will keep it for eternal life.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 13,
                start_verse: 34,
                end_verse: 35,
            },
            text: "A new commandment I give to you, that you love one another:  just as I have loved you, you also are to love one another.  By this all people will know that you are my disciples, if you have love for one another.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 14,
                start_verse: 1,
                end_verse: 3,
            },
            text: "Let not your hearts be troubled.  Believe in God; believe also in me.  In my Father's house are many rooms.  If it were not so, would I have told you that I go to prepare a place for you?  And if I go and prepare a place for you, I will come again and will take you to myself, that where I am you may be also.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 14,
                start_verse: 6,
                end_verse: 6,
            },
            text: "Jesus said to him, “I am the way, and the truth, and the life.  No one comes to the Father except through me.”",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 14,
                start_verse: 27,
                end_verse: 27,
            },
            text: "Peace I leave with you; my peace I give to you.  Not as the world gives do I give to you.  Let not your hearts be troubled, neither let them be afraid.",
        }, Verse {
            reference: Reference {
                book: Book::John,
                chapter: 15,
                start_verse: 5,
                end_verse: 5,
            },
            text: "I am the vine; you are the branches.  Whoever abides in me and I in him, he it is that bears much fruit, for apart from me you can do nothing.",
        }, 
        Verse {
            reference: Reference {
                book: Book::John,
                chapter: 16,
                start_verse: 33,
                end_verse: 33,
            },
            text: "I have said these things to you, that in me you may have peace.  In the world you will have tribulation.  But take heart; I have overcome the world.",
        }, Verse {
            reference: Reference {
                book: Book::Acts,
                chapter: 1,
                start_verse: 8,
                end_verse: 8,
            },
            text: "But you will receive power when the Holy Spirit has come upon you, and you will be my witnesses in Jerusalem and in all Judea and Samaria, and to the end of the earth.",
        }, Verse {
            reference: Reference {
                book: Book::Acts,
                chapter: 3,
                start_verse: 19,
                end_verse: 20,
            },
            text: "Repent therefore, and turn back, that your sins may be blotted out, that times of refreshing may come from the presence of the Lord.",
        }, Verse {
            reference: Reference {
                book: Book::Acts,
                chapter: 4,
                start_verse: 12,
                end_verse: 12,
            },
            text: "And there is salvation in no one else, for there is no other name under heaven given among men by which we must be saved.",
        }, Verse {
            reference: Reference {
                book: Book::Acts,
                chapter: 20,
                start_verse: 24,
                end_verse: 24,
            },
            text: "But I do not account my life of any value nor as precious to myself, if only I may finish my course and the ministry that I received from the Lord Jesus, to testify to the gospel of the grace of God.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 1,
                start_verse: 16,
                end_verse: 16,
            },
            text: "For I am not ashamed of the gospel, for it is the power of God for salvation to everyone who believes, to the Jew first and also to the Greek. ",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 2,
                start_verse: 4,
                end_verse: 4,
            },
            text: "Or do you presume on the riches of his kindness and forbearance and patience, not knowing that God's kindness is meant to lead you to repentance?",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 3,
                start_verse: 23,
                end_verse: 23,
            },
            text: "For all have sinned and fall short of the glory of God.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 5,
                start_verse: 6,
                end_verse: 10,
            },
            text: "For while we were still weak, at the right time Christ died for the ungodly.  For one will scarcely die for a righteous person -- though perhaps for a good person one would dare even to die -- but God shows his love for us in that while we were still sinners, Christ died for us.  Since, therefore, we have now been justified by his blood, much more shall we be saved by him from the wrath of God.  For if while we were enemies we were reconciled to God by the death of his Son, much more, now that we are reconciled, shall we be saved by his life.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 6,
                start_verse: 12,
                end_verse: 13,
            },
            text: "Let not sin therefore reign in your mortal body, to make you obey its passions.  Do not present your members to sin as instruments for unrighteousness, but present yourselves to God as those who have been brought from death to life, and your members to God as instruments for righteousness.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 6,
                start_verse: 23,
                end_verse: 23,
            },
            text: "For the wages of sin is death, but the free gift of God is eternal life in Christ Jesus our Lord.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 8,
                start_verse: 1,
                end_verse: 2,
            },
            text: "There is therefore now no condemnation for those who are in Christ Jesus.  For the law of the Spirit of life has set you free in Christ Jesus from the law of sin and death.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 8,
                start_verse: 15,
                end_verse: 16,
            },
            text: "For you did not receive the spirit of slavery to fall back into fear, but you received the Spirit of adoption as sons, by whom we cry, “Abba!  Father!”  The Spirit himself bears witness with our spirit that we are children of God.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 8,
                start_verse: 28,
                end_verse: 28,
            },
            text: "And we know that for those who love God all things work together for good, for those who are called according to his purpose.",
        }, 
        
        Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 8,
                start_verse: 37,
                end_verse: 39,
            },
            text: "No, in all these things we are more than conquerors through him who loved us.  For I am sure that neither death nor life, nor angels nor rulers, nor things present nor things to come, nor powers, nor height nor depth, nor anything else in all creation, will be able to separate us from the love of God in Christ Jesus our Lord.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 10,
                start_verse: 9,
                end_verse: 10,
            },
            text: "Because, if you confess with your mouth that Jesus is Lord and believe in your heart that God raised him from the dead, you will be saved.  For with the heart one believes and is justified, and with the mouth one confesses and is saved.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 12,
                start_verse: 1,
                end_verse: 2,
            },
            text: "I appeal to you therefore, brothers, by the mercies of God, to present your bodies as a living sacrifice, holy and acceptable to God, which is your spiritual worship.  Do not be conformed to this world, but be transformed by the renewal of your mind, that by testing you may discern what is the will of God, what is good and acceptable and perfect.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 13,
                start_verse: 8,
                end_verse: 8,
            },
            text: "Owe no one anything, except to love each other, for the one who loves another has fulfilled the law.",
        }, Verse {
            reference: Reference {
                book: Book::Romans,
                chapter: 14,
                start_verse: 8,
                end_verse: 8,
            },
            text: "For if we live, we live to the Lord, and if we die, we die to the Lord.  So then, whether we live or whether we die, we are the Lord’s.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 1,
                start_verse: 18,
                end_verse: 18,
            },
            text: "For the word of the cross is folly to those who are perishing, but to us who are being saved it is the power of God.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 2,
                start_verse: 2,
                end_verse: 2,
            },
            text: "For I decided to know nothing among you except Jesus Christ and him crucified.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 2,
                start_verse: 14,
                end_verse: 14,
            },
            text: "The natural person does not accept the things of the Spirit of God, for they are folly to him, and he is not able to understand them because they are spiritually discerned.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 6,
                start_verse: 19,
                end_verse: 20,
            },
            text: "Or do you not know that your body is a temple of the Holy Spirit within you, whom you have from God?  You are not your own, for you were bought with a price.  So glorify God in your body.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 9,
                start_verse: 22,
                end_verse: 22,
            },
            text: "I have become all things to all people, that by all means I might save some.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 9,
                start_verse: 26,
                end_verse: 27,
            },
            text: "So I do not run aimlessly; I do not box as one beating the air.  But I discipline my body and keep it under control, lest after preaching to others I myself should be disqualified.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 10,
                start_verse: 13,
                end_verse: 13,
            },
            text: "No temptation has overtaken you that is not common to man.  God is faithful, and he will not let you be tempted beyond your ability, but with the temptation he will also provide the way of escape, that you may be able to endure it.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 12,
                start_verse: 18,
                end_verse: 18,
            },
            text: "But as it is, God arranged the members in the body, each one of them, as he chose.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 13,
                start_verse: 13,
                end_verse: 13,
            },
            text: "So now faith, hope, and love abide, these three; but the greatest of these is love.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians1,
                chapter: 15,
                start_verse: 58,
                end_verse: 58,
            },
            text: "Therefore, my beloved brothers, be steadfast, immovable, always abounding in the work of the Lord, knowing that in the Lord your labor is not in vain.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 3,
                start_verse: 5,
                end_verse: 5,
            },
            text: "Not that we are sufficient in ourselves to claim anything as coming from us, but our sufficiency is from God.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 3,
                start_verse: 17,
                end_verse: 17,
            },
            text: "Now the Lord is the Spirit, and where the Spirit of the Lord is, there is freedom.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 4,
                start_verse: 4,
                end_verse: 4,
            },
            text: "In their case the god of this world has blinded the minds of the unbelievers, to keep them from seeing the light of the gospel of the glory of Christ, who is the image of God.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 4,
                start_verse: 7,
                end_verse: 7,
            },
            text: "But we have this treasure in jars of clay, to show that the surpassing power belongs to God and not to us.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 4,
                start_verse: 16,
                end_verse: 18,
            },
            text: "So we do not lose heart.  Though our outer self is wasting away, our inner self is being renewed day by day.  For this light momentary affliction is preparing for us an eternal weight of glory beyond all comparison, as we look not to the things that are seen but to the things that are unseen.  For the things that are seen are transient, but that things that are unseen are eternal.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 5,
                start_verse: 7,
                end_verse: 7,
            },
            text: "For we walk by faith, not by sight.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 5,
                start_verse: 14,
                end_verse: 14,
            },
            text: "For the love of Christ controls us, because we have concluded this:  that one has died for all, therefore all have died.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 5,
                start_verse: 17,
                end_verse: 17,
            },
            text: "Therefore, if anyone is in Christ, he is a new creation.  The old has passed away; behold, the new has come.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 6,
                start_verse: 14,
                end_verse: 14,
            },
            text: "Do not be equally yoked with unbelievers.  For what partnership has righteousness with lawlessness?  Or what fellowship has light with darkness?",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 7,
                start_verse: 10,
                end_verse: 10,
            },
            text: "For godly grief produces a repentance that leads to salvation without regret, whereas worldly grief produces death.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 9,
                start_verse: 7,
                end_verse: 7,
            },
            text: "Each man must give as he has decided in his heart, not reluctantly or under compulsion, for God loves a cheerful giver.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 11,
                start_verse: 14,
                end_verse: 14,
            },
            text: "And no wonder, for even Satan disguises himself as an angel of light.",
        }, Verse {
            reference: Reference {
                book: Book::Corinthians2,
                chapter: 12,
                start_verse: 9,
                end_verse: 10,
            },
            text: "But he said to me, “My grace is sufficient for you, for my power is made perfect in weakness.”  Therefore I will boast all the more gladly of my weaknesses, so that the power of Christ may rest upon me.  For the sake of Christ, then, I am content weaknesses, insults, hardships, persecutions, and calamities.  For when I am weak, then I am strong.",
        }, Verse {
            reference: Reference {
                book: Book::Galatians,
                chapter: 1,
                start_verse: 10,
                end_verse: 10,
            },
            text: "For am I now seeking the approval of man, or of God?  Or am I trying to please man?  If I were still trying to please man, I would not be a servant of Christ.",
        }, Verse {
            reference: Reference {
                book: Book::Galatians,
                chapter: 2,
                start_verse: 20,
                end_verse: 20,
            },
            text: "I have been crucified with Christ.  It is no longer I who live, but Christ who lives in me.  And the life I now live in the flesh I live by faith in the Son of God, who loved me and gave himself for me.",
        }, Verse {
            reference: Reference {
                book: Book::Galatians,
                chapter: 5,
                start_verse: 16,
                end_verse: 17,
            },
            text: "But I say, walk by the Spirit, and you will not gratify the desires of the flesh.  For the desires of the flesh are against the Spirit, and the desires of the Spirit are against the flesh, for these are opposed to each other, to keep you from doing the things you want to do.",
        }, Verse {
            reference: Reference {
                book: Book::Galatians,
                chapter: 5,
                start_verse: 22,
                end_verse: 23,
            },
            text: "But the fruit of the Spirit is love, joy, peace, patience, kindness, goodness, faithfulness, gentleness, self-control; against such things there is no law.",
        }, Verse {
            reference: Reference {
                book: Book::Galatians,
                chapter: 6,
                start_verse: 2,
                end_verse: 2,
            },
            text: "Bear one another’s burdens, and so fulfill the law of Christ.",
        }, Verse {
            reference: Reference {
                book: Book::Galatians,
                chapter: 6,
                start_verse: 7,
                end_verse: 9,
            },
            text: "Do not be deceived:  God is not mocked, for whatever one sows, that will he also reap.  For the one who sows to his own flesh will from the flesh reap corruption, but the one who sows to the Spirit will from the Spirit reap eternal life.  And let us not grow weary of doing good, for in due season we will reap, if we do not give up.",
        }, Verse {
            reference: Reference {
                book: Book::Galatians,
                chapter: 6,
                start_verse: 14,
                end_verse: 14,
            },
            text: "But far be it from me to boast except in the cross of our Lord Jesus Christ, by which the world has been crucified to me, and I to the world.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 1,
                start_verse: 3,
                end_verse: 3,
            },
            text: "Blessed be the God and Father of our Lord Jesus Christ, who has blessed us in Christ with every spiritual blessing in the heavenly places.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 2,
                start_verse: 8,
                end_verse: 10,
            },
            text: "For by grace you have been saved through faith.  And this is not your own doing; it is the gift of God, not a result of works, so that no one may boast.  For we are his workmanship, created in Christ Jesus for good works, which God prepared beforehand, that we should walk in them.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 2,
                start_verse: 14,
                end_verse: 14,
            },
            text: "For he himself is our peace, who has made us both one and has broken down in his flesh the dividing wall of hostility.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 3,
                start_verse: 16,
                end_verse: 21,
            },
            text: "That according to the riches of his glory he may grant you to be strengthened with power through his Spirit in your inner being, so that Christ may dwell in your hearts through faith -- that you, being rooted and grounded in love, may have strength to comprehend with all the saints what is the breadth and length and height and depth, and to know the love of Christ that surpasses knowledge, that you may be filled with all the fullness of God.  Now to him who is able to do far more abundantly than all that we ask or think, according to the power at work within us, to him be glory in the church and in Christ Jesus throughout all generations, forever and ever.  Amen.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 4,
                start_verse: 11,
                end_verse: 12,
            },
            text: "And he gave the apostles, the prophets, the evangelists, the shepherds and teachers, to equip the saints for the work of ministry, for building up the body of Christ.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 4,
                start_verse: 22,
                end_verse: 24,
            },
            text: "To put off your old self, which belongs to your former manner of life and is corrupt through deceitful desires, and to be renewed in the spirit of your minds, and to put on the new self, created after the likeness of God in true righteousness and holiness.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 5,
                start_verse: 1,
                end_verse: 2,
            },
            text: "Therefore be imitators of God, as beloved children.  And walk in love, as Christ loved us and gave himself up for us, a fragrant offering and sacrifice to God.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 5,
                start_verse: 11,
                end_verse: 11,
            },
            text: "Take no part in the unfruitful works of darkness, but instead expose them.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 5,
                start_verse: 15,
                end_verse: 16,
            },
            text: "Look carefully then how you walk, not as unwise but as wise, making the best use of the time, because the days are evil.",
        }, Verse {
            reference: Reference {
                book: Book::Ephesians,
                chapter: 6,
                start_verse: 10,
                end_verse: 12,
            },
            text: "Finally, be strong in the Lord and in the strength of his might.  Put on the whole armor of God, that you may be able to stand against the schemes of the devil.  For we do not wrestle against flesh and blood, but against the rulers, against the authorities, against the cosmic powers over this present darkness, against the spiritual forces of evil in the heavenly places.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 1,
                start_verse: 6,
                end_verse: 6,
            },
            text: "And I am sure of this, that he who began a good work in you will bring it to completion at the day of Jesus Christ.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 1,
                start_verse: 21,
                end_verse: 21,
            },
            text: "For to me to live is Christ, and to die is gain.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 2,
                start_verse: 1,
                end_verse: 11,
            },
            text: "So if there is any encouragement in Christ, any comfort from love, any participation in the Spirit, any affection and sympathy, complete my joy by being of the same mind, having the same love, being in full accord and of one mind.  Do nothing from selfish ambition or conceit, but in humility count others more significant than yourselves.  Let each of you look not only to his own interests, but also to the interests of others.  Have this mind among yourselves, which is yours in Christ Jesus, who, though he was in the form of God, did not count equality with God a thing to be grasped, but emptied himself, by taking the form of a servant, being born in the likeness of men.  And being found in human form, he humbled himself by becoming obedient to the point of death, even death on a cross.  Therefore God has highly exalted him and bestowed on him the name that is above every name, so that at the name of Jesus every knee should bow, in heaven and on earth and under the earth, and every tongue confess that Jesus Christ is Lord, to the glory of God the Father.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 2,
                start_verse: 13,
                end_verse: 13,
            },
            text: "For it is God who works in you, both to will and to work for his good pleasure.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 3,
                start_verse: 7,
                end_verse: 8,
            },
            text: "But whatever gain I had, I counted as loss for the sake of Christ.  Indeed, I count everything as loss because of the surpassing worth of knowing Christ Jesus my Lord.  For his sake I have suffered the loss of all things and count them as rubbish, in order that I may gain Christ.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 3,
                start_verse: 13,
                end_verse: 14,
            },
            text: "Brothers, I do not consider that I have made it my own.  But one thing I do:  forgetting what lies behind and straining forward to what lies ahead, I press on toward the goal for the prize of the upward call of God in Christ Jesus.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 4,
                start_verse: 6,
                end_verse: 7,
            },
            text: "Do not be anxious about anything, but in everything by prayer and supplication with thanksgiving let your requests be made known to God.  And the peace of God, which surpasses all understanding, will guard your hearts and your minds in Christ Jesus.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 4,
                start_verse: 8,
                end_verse: 8,
            },
            text: "Finally, brothers, whatever is true, whatever is honorable, whatever is just, whatever is pure, whatever is lovely, whatever is commendable, if there is any excellence, if there is anything worthy of praise, think about these things.",
        }, Verse {
            reference: Reference {
                book: Book::Philippians,
                chapter: 4,
                start_verse: 19,
                end_verse: 19,
            },
            text: "And my God will supply every need of yours according to his riches in glory in Christ Jesus.",
        }, Verse {
            reference: Reference {
                book: Book::Colossians,
                chapter: 3,
                start_verse: 1,
                end_verse: 2,
            },
            text: "If then you have been raised with Christ, seek the things that are above, where Christ is, seated at the right hand of God.  Set your minds on things that are above, not on things that are on earth.",
        }, Verse {
            reference: Reference {
                book: Book::Thessalonians1,
                chapter: 5,
                start_verse: 16,
                end_verse: 18,
            },
            text: "Rejoice always, pray without ceasing, give thanks in all circumstances; for this is the will of God in Christ Jesus for you.",
        }, Verse {
            reference: Reference {
                book: Book::Timothy1,
                chapter: 6,
                start_verse: 6,
                end_verse: 8,
            },
            text: "But godliness with contentment is great gain, for we brought nothing into the world, and we cannot take anything out of the world.  But if we have food and clothing, with these we will be content.",
        }, Verse {
            reference: Reference {
                book: Book::Timothy2,
                chapter: 1,
                start_verse: 7,
                end_verse: 7,
            },
            text: "For God gave us a spirit of not of fear but of power and love and self-control.",
        }, 
        
        Verse {
            reference: Reference {
                book: Book::Timothy2,
                chapter: 1,
                start_verse: 12,
                end_verse: 12,
            },
            text: "Which is why I suffer as I do.  But I am not ashamed, for I know whom I have believed, and I am convinced that he is able to guard until that Day what has been entrusted to me.",
        }, Verse {
            reference: Reference {
                book: Book::Timothy2,
                chapter: 2,
                start_verse: 3,
                end_verse: 4,
            },
            text: "Share in suffering as a good soldier of Christ Jesus.  No soldier gets entangled in civilian pursuits, since his aim is to please the one who enlisted him.",
        }, Verse {
            reference: Reference {
                book: Book::Timothy2,
                chapter: 2,
                start_verse: 21,
                end_verse: 21,
            },
            text: "Therefore, if anyone cleanses himself from what is dishonorable, he will be a vessel for honorable use, set apart as holy, useful to the master of the house, ready for every good work.",
        }, Verse {
            reference: Reference {
                book: Book::Timothy2,
                chapter: 3,
                start_verse: 16,
                end_verse: 17,
            },
            text: "All Scripture is breathed out by God and profitable for teaching, for reproof, for correction, and for training in righteousness, that the man of God may be complete, equipped for every good work.",
        }, Verse {
            reference: Reference {
                book: Book::Timothy2,
                chapter: 4,
                start_verse: 6,
                end_verse: 8,
            },
            text: "For I am already being poured out as a drink offering, and the time of my departure has come.  I have fought the good fight, I have finished the race, I have kept the faith.  Henceforth there is laid up for me the crown of righteousness, which the Lord, the righteous judge, will award to me on that Day, and not only to me but also to all who have loved his appearing.",
        }, Verse {
            reference: Reference {
                book: Book::Hebrews,
                chapter: 4,
                start_verse: 12,
                end_verse: 12,
            },
            text: "For the word of God is living and active, sharper than any two-edged sword, piercing to the division of soul and of spirit, of joints and of marrow, and discerning the thoughts and intentions of the heart.",
        }, Verse {
            reference: Reference {
                book: Book::Hebrews,
                chapter: 4,
                start_verse: 15,
                end_verse: 16,
            },
            text: "For we do not have a high priest who is unable to sympathize with our weaknesses, but one who in every respect has been tempted as we are, yet without sin.  Let us then with confidence draw near to the throne of grace, that we may receive mercy and find grace to help in time of need.",
        }, Verse {
            reference: Reference {
                book: Book::Hebrews,
                chapter: 10,
                start_verse: 23,
                end_verse: 25,
            },
            text: "Let us hold fast the confession of our hope without wavering, for he who promised is faithful.  And let us consider how to stir up one another to love and good works, not neglecting to meet together, as is the habit of some, but encouraging one another, and all the more as you see the Day drawing near.",
        }, Verse {
            reference: Reference {
                book: Book::Hebrews,
                chapter: 11,
                start_verse: 1,
                end_verse: 1,
            },
            text: "Now faith is the assurance of things hoped for, the conviction of things not seen.",
        }, Verse {
            reference: Reference {
                book: Book::Hebrews,
                chapter: 11,
                start_verse: 6,
                end_verse: 6,
            },
            text: "And without faith it is impossible to please him, for whoever would draw near to God must believe that he exists and that he rewards those who seek him.",
        }, Verse {
            reference: Reference {
                book: Book::Hebrews,
                chapter: 12,
                start_verse: 1,
                end_verse: 2,
            },
            text: "Therefore, since we are surrounded by so great a cloud of witnesses, let us also lay aside every weight, and sin which clings so closely, and let us run with endurance the race that is set before us, looking to Jesus, the founder and perfecter of our faith, who for the joy that was set before him endured the cross, despising the shame, and is seated at the right hand of the throne of God.",
        }, Verse {
            reference: Reference {
                book: Book::Hebrews,
                chapter: 12,
                start_verse: 4,
                end_verse: 4,
            },
            text: "In your struggle against sin you have not yet resisted to the point of shedding your blood.",
        }, Verse {
            reference: Reference {
                book: Book::Hebrews,
                chapter: 12,
                start_verse: 11,
                end_verse: 11,
            },
            text: "For the moment all discipline seems painful rather than pleasant, but later it yields the peaceful fruit of righteousness to those who have been trained by it.",
        }, Verse {
            reference: Reference {
                book: Book::James,
                chapter: 4,
                start_verse: 7,
                end_verse: 7,
            },
            text: "Submit yourselves therefore to God.  Resist the devil, and he will flee from you.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 1,
                start_verse: 18,
                end_verse: 19,
            },
            text: "Knowing that you were ransomed from the futile ways inherited from your forefathers, not with perishable things such as silver or gold, but with the precious blood of Christ, like that of a lamb without blemish or spot.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 2,
                start_verse: 9,
                end_verse: 9,
            },
            text: "But you are a chosen race, a royal priesthood, a holy nation, a people for his own possession, that you may proclaim the excellencies of him who called you out of darkness into his marvelous light.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 2,
                start_verse: 11,
                end_verse: 12,
            },
            text: "Beloved, I urge you as sojourners and exiles to abstain from the passions of the flesh, which wage war against your soul.  Keep your conduct among the Gentiles honorable, so that when they speak against you as evildoers, they may see your good deeds and glorify God on the day of visitation.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 2,
                start_verse: 21,
                end_verse: 21,
            },
            text: "For to this you have been called, because Christ also suffered for you, leaving you an example, so that you might follow in his steps.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 3,
                start_verse: 15,
                end_verse: 16,
            },
            text: "But in your hearts honor Christ the Lord as holy, always being prepared to make a defense to anyone who asks you for a reason for the hope that is in you; yet do it with gentleness and respect, having a good conscience, so that, when you are slandered, those who revile your good behavior in Christ may be put to shame.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 4,
                start_verse: 7,
                end_verse: 8,
            },
            text: "The end of all things is at hand; therefore be self-controlled and sober-minded for the sake of your prayers.  Above all, keep loving one another earnestly, since love covers a multitude of sins.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 4,
                start_verse: 10,
                end_verse: 10,
            },
            text: "As each has received a gift, use it to serve one another, as good stewards of God’s varied grace.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 5,
                start_verse: 6,
                end_verse: 7,
            },
            text: "Humble yourselves, therefore, under the mighty hand of God so that at the proper time he may exalt you, casting all your anxieties on him, because he cares for you.",
        }, Verse {
            reference: Reference {
                book: Book::Peter1,
                chapter: 5,
                start_verse: 8,
                end_verse: 9,
            },
            text: "Be sober-minded; be watchful.  Your adversary the devil prowls around like a roaring lion, seeking someone to devour.  Resist him, firm in your faith, knowing that the same kinds of suffering are being experienced by your brotherhood throughout the world.",
        }, Verse {
            reference: Reference {
                book: Book::Peter2,
                chapter: 3,
                start_verse: 8,
                end_verse: 9,
            },
            text: "But do not overlook this one fact, beloved, that with the Lord one day is as a thousand years, and a thousand years as one day.  The Lord is not slow to fulfill his promise as some count slowness, but is patient toward you, not wishing that any should perish, but that all should reach repentance.",
        }, Verse {
            reference: Reference {
                book: Book::John1,
                chapter: 1,
                start_verse: 9,
                end_verse: 9,
            },
            text: "If we confess our sins, he is faithful and just to forgive us our sins and to cleanse us from all unrighteousness.",
        }, Verse {
            reference: Reference {
                book: Book::John1,
                chapter: 2,
                start_verse: 15,
                end_verse: 16,
            },
            text: "Do not love the world or the things in the world.  If anyone loves the world, the love of the Father is not in him.  For all that is in the world -- the desires of the flesh and the desires of the eyes and pride of life -- is not from the Father but is from the world.",
        }, Verse {
            reference: Reference {
                book: Book::John1,
                chapter: 3,
                start_verse: 16,
                end_verse: 16,
            },
            text: "By this we know love, that he laid down his life for us, and we ought to lay down our lives for the brothers.",
        }, Verse {
            reference: Reference {
                book: Book::John1,
                chapter: 4,
                start_verse: 4,
                end_verse: 4,
            },
            text: "Little children, you are from God and have overcome them, for he who is in you is greater than he who is in the world.",
        }, 
        
        Verse {
            reference: Reference {
                book: Book::John1,
                chapter: 4,
                start_verse: 17,
                end_verse: 19,
            },
            text: "By this is love perfected with us, so that we may have confidence for the day of judgment, because as he is so also are we in this world.  There is no fear in love, but perfect love casts out fear.  For fear has to do with punishment, and whoever fears has not been perfected in love.  We love because he first loved us.",
        }, Verse {
            reference: Reference {
                book: Book::John1,
                chapter: 5,
                start_verse: 3,
                end_verse: 3,
            },
            text: "For this is the love of God, that we keep his commandments.  And his commandments are not burdensome.",
        }, Verse {
            reference: Reference {
                book: Book::John1,
                chapter: 5,
                start_verse: 13,
                end_verse: 15,
            },
            text: "I write these things to you who believe in the name of the Son of God that you may know that you have eternal life.  And this is the confidence that we have toward him, that if we ask anything according to his will he hears us.  And if we know that he hears us in whatever we ask, we know that we have the requests that we have asked of him.",
        }, Verse {
            reference: Reference {
                book: Book::Revelation,
                chapter: 3,
                start_verse: 20,
                end_verse: 20,
            },
            text: "Behold, I stand at the door and knock.  If anyone hears my voice and opens the door, I will come in to him and eat with him, and he with me.",
        }  
    ];

pub fn get_random_verse() -> Verse {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..VERSES.len());
    VERSES[index].clone()
}
