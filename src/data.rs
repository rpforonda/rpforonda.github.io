//! All site content lives here. Edit this file to make the portfolio yours.

pub const NAME: &str = "Roel Foronda";
pub const ROLE: &str = "Security-Focused Web & Power Platform Developer";
pub const TAGLINE: &str = "16+ years in IT and cybersecurity, now building secure web apps, SharePoint solutions and automations with security designed in from day one.";
pub const LOCATION: &str = "Germany";
pub const EMAIL: &str = "rpforonda@gmail.com";
pub const GITHUB: &str = "https://github.com/rpforonda";
pub const LINKEDIN: &str = "https://www.linkedin.com/in/rpforonda";
/// The résumé isn't published; visitors request it by email instead.
pub const RESUME_REQUEST_SUBJECT: &str = "Résumé request";

pub const ABOUT: &[&str] = &[
    "I'm an IT professional with more than 16 years of experience, \
     starting with a decade in the US Marine Corps running networks and cybersecurity programs, \
     then leading system accreditation and security management at Booz Allen Hamilton and Lockheed Martin.",
    "My passion for technology pulled me into software engineering. Today I build SharePoint \
     applications, Svelte and Next.js front-ends, and Power Platform solutions, and I bring a \
     security background to every project: security belongs at the start of the development life cycle, not bolted on at the end.",
    "I like breaking down the silos between cybersecurity and engineering teams so they \
     collaborate instead of hand off.",
];

pub struct Stat {
    pub value: &'static str,
    pub label: &'static str,
}

pub const STATS: &[Stat] = &[
    Stat {
        value: "16+ yrs",
        label: "IT & cybersecurity",
    },
    Stat {
        value: "CISSP",
        label: "Credential ID 587266",
    },
    Stat {
        value: "10 yrs",
        label: "US Marine Corps",
    },
];

pub struct SkillGroup {
    pub title: &'static str,
    pub icon: &'static str,
    pub items: &'static [&'static str],
}

pub const SKILLS: &[SkillGroup] = &[
    SkillGroup {
        title: "Power Platform & M365",
        icon: "▣",
        items: &[
            "Power Apps",
            "Power Automate",
            "SharePoint REST API",
            "PnPjs",
            "Dataverse",
        ],
    },
    SkillGroup {
        title: "Front-end",
        icon: "{ }",
        items: &[
            "Svelte 5 / SvelteKit",
            "React",
            "Next.js",
            "TypeScript",
            "Tailwind CSS",
            "Leaflet maps",
            "jQuery → modern JS migrations",
        ],
    },
    SkillGroup {
        title: "Back-end & Data",
        icon: "◈",
        items: &[
            "C# / ASP.NET MVC & Web API",
            "Entity Framework / ADO.NET",
            "SQL Server",
            "Stored procedures",
            "REST API design",
        ],
    },
    SkillGroup {
        title: "Tooling & Delivery",
        icon: "⟲",
        items: &[
            "Git & GitHub",
            "Agile / code reviews",
            "GitHub Actions CI/CD",
            "GitHub Pages / static hosting",
            "Vite",
            "Rust + WASM (this site!)",
        ],
    },
    SkillGroup {
        title: "Security",
        icon: "⛨",
        items: &[
            "RMF / NIST / FedRAMP",
            "ATO & security documentation",
            "Tenable Nessus",
            "Splunk / CloudTrail",
            "Incident response",
        ],
    },
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Category {
    #[allow(dead_code)]
    // no Power Platform projects listed yet; its filter appears once one is added
    PowerPlatform,
    SharePoint,
    Web,
    FullStack,
}

impl Category {
    pub const ALL: [Category; 4] = [
        Category::Web,
        Category::SharePoint,
        Category::FullStack,
        Category::PowerPlatform,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Category::PowerPlatform => "Power Platform",
            Category::SharePoint => "SharePoint",
            Category::Web => "Web / JS",
            Category::FullStack => "Full-stack",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Category::PowerPlatform => "tag-apps",
            Category::SharePoint => "tag-automate",
            Category::Web => "tag-js",
            Category::FullStack => "tag-full",
        }
    }
}

pub struct Project {
    pub id: u32,
    pub title: &'static str,
    pub category: Category,
    pub problem: &'static str,
    pub solution: &'static str,
    pub result: &'static str,
    pub stack: &'static [&'static str],
    /// (label, url) pairs shown at the bottom of the card.
    pub links: &'static [(&'static str, &'static str)],
    /// Placeholder content you still need to replace. Shows a "Sample" badge.
    pub sample: bool,
}

pub const PROJECTS: &[Project] = &[
    Project {
        id: 1,
        title: "Catch Up With Alice: Author Website",
        category: Category::Web,
        problem: "A children's book author needed a fast, low-maintenance home for books, freebies and a newsletter, with no servers to run.",
        solution: "SvelteKit + Svelte 5 runes site, fully prerendered for SEO, with optimized AVIF/WebP images, downloadable coloring pages and \"Cosmic Dash\", a custom endless-runner browser game starring the book's characters.",
        result: "Live in production on a custom domain, deployed automatically by GitHub Actions, and $0 hosting.",
        stack: &["SvelteKit", "Svelte 5", "Tailwind v4", "Vite", "GitHub Pages"],
        links: &[
            ("Live site", "https://catchupwithalice.com"),
            ("Code", "https://github.com/rpforonda/byaliceforonda"),
        ],
        sample: false,
    },
    Project {
        // NOTE: private repo for a government program, so keep the description generic.
        id: 2,
        title: "Interoperability Capability Map",
        category: Category::SharePoint,
        problem: "A defense organization tracked partner capability data in SharePoint lists with no way to see it geographically.",
        solution: "Interactive Leaflet map in Next.js/TypeScript that plots SharePoint list data, opens detailed record views in modals, resolves editor names via the SharePoint REST API and prints formatted reports.",
        result: "Turned a flat list into a visual, printable briefing tool that improved operational efficiency and data accessibility.",
        stack: &["Next.js", "TypeScript", "React Leaflet", "SharePoint REST", "Tailwind"],
        links: &[],
        sample: false,
    },
    Project {
        id: 7,
        title: "SharePoint Portal Modernization",
        category: Category::SharePoint,
        problem: "A defense organization's SharePoint portal ran on aging jQuery code and dated styling.",
        solution: "Migrated the front-end from jQuery to modern JavaScript and Svelte components, and rebuilt the portal's CSS for a contemporary, responsive design.",
        result: "A more secure, faster and more user-friendly portal for its users.",
        stack: &["Svelte", "JavaScript", "SharePoint", "CSS"],
        links: &[],
        sample: false,
    },
    Project {
        id: 3,
        title: "SupplyLocker",
        category: Category::SharePoint,
        problem: "Supply requests ran through email and spreadsheets with no catalog or status tracking.",
        solution: "Next.js 15 / React 19 storefront with product categories, cart, order status and an admin dashboard, using SharePoint (via PnPjs) as the data layer.",
        result: "In progress.", // TODO: update when shipped
        stack: &["Next.js 15", "React 19", "TypeScript", "PnPjs", "Tailwind"],
        links: &[],
        sample: false,
    },
    Project {
        id: 4,
        title: "User Profile & Auth Module",
        category: Category::FullStack,
        problem: "Freelance agile team project: users needed to securely view and update their profiles and change passwords.",
        solution: "End-to-end feature: SQL Server stored procedures, a C# Web API controller with a service layer, and React forms with schema validation, pagination and survey components.",
        result: "One feature built across every layer of the stack.",
        stack: &["React", "C#", "ASP.NET Web API", "SQL Server"],
        links: &[
            ("React", "https://github.com/rpforonda/React"),
            ("C# API", "https://github.com/rpforonda/Middle-Tier"),
            ("SQL", "https://github.com/rpforonda/Back-End"),
        ],
        sample: false,
    },
];

pub struct Cert {
    pub code: &'static str,
    pub name: &'static str,
    pub issuer: &'static str,
    pub status: &'static str,
    pub url: &'static str,
}

pub const CERTS: &[Cert] = &[
    Cert {
        code: "CISSP",
        name: "Certified Information Systems Security Professional",
        issuer: "ISC2",
        status: "Since 2016 · Active through 2028",
        url: "https://www.isc2.org/MemberVerification",
    },
    Cert {
        code: "PL-900",
        name: "Microsoft Certified: Power Platform Fundamentals",
        issuer: "Microsoft",
        status: "Issued Oct 2022",
        url: "https://learn.microsoft.com/credentials/", // TODO: paste your "Show credential" link
    },
    // CompTIA Security+ (2014) expired Sep 2026, so it's left off. Re-add it here if you renew.
];

pub struct Job {
    pub title: &'static str,
    pub org: &'static str,
    pub dates: &'static str,
    pub location: &'static str,
    pub bullets: &'static [&'static str],
}

pub const EXPERIENCE: &[Job] = &[
    Job {
        title: "SharePoint Developer · IM/KM Specialist",
        org: "C4 Planning Solutions",
        dates: "Jul 2022 – Present",
        location: "Germany",
        bullets: &[
            "Develop and maintain SharePoint sites and applications for a US Department of Defense customer.",
            "Migrated the portal from jQuery to modern JavaScript and Svelte to improve security and performance, and modernized its CSS.",
            "Built an interactive mapping application in Next.js, improving operational efficiency and data accessibility.",
            "Gather requirements with cross-functional teams and deliver custom SharePoint solutions and automations.",
        ],
    },
    Job {
        title: "Software Engineer",
        org: "Freelance",
        dates: "Jan 2022 – Jul 2022",
        location: "Los Angeles, CA",
        bullets: &[
            "Built features in React and C#/.NET (MVC, Entity Framework, ADO.NET, SQL) on an agile team.",
            "Designed and implemented secure end-user profile editing, end to end.",
            "Set up source control and workflow management with GitHub, Git and Trello.",
        ],
    },
    Job {
        title: "Classified Cyber Security Staff",
        org: "Lockheed Martin",
        dates: "Nov 2020 – Jan 2022",
        location: "Palmdale, CA",
        bullets: &[
            "Assessed the security posture of classified programs through configuration audits and vulnerability assessments.",
            "Applied the NIST Risk Management Framework to define technical security requirements and authored authorization documentation.",
            "Ran incident response investigations and POA&M tracking.",
        ],
    },
    Job {
        title: "Associate, Cybersecurity",
        org: "Booz Allen Hamilton",
        dates: "Jun 2019 – Oct 2020",
        location: "Chantilly, VA",
        bullets: &[
            "Led accreditation for AWS-based cloud and on-prem programs under FISMA, FedRAMP and NIST guidance.",
            "Produced vulnerability and compliance metrics with Splunk, CloudWatch, CloudTrail and Nessus.",
            "Worked with developers and infrastructure teams to obtain authorization to operate on federal networks.",
        ],
    },
    Job {
        title: "Cybersecurity Manager · Cybersecurity Technician · Data Network Specialist",
        org: "US Marine Corps",
        dates: "Jul 2009 – Aug 2019",
        location: "Asia-Pacific · Deployed",
        bullets: &[
            "Led a command cybersecurity section, including user training and leadership awareness.",
            "Implemented and monitored security measures for regional command networks.",
            "Installed and managed networks and security devices in austere tactical environments; supported 300+ users.",
        ],
    },
];

/// Featured project with a playable in-browser build (see play/ and scripts/build-game.sh).
pub struct Game {
    pub title: &'static str,
    pub pitch: &'static str,
    pub repo: &'static str,
    pub play_url: &'static str,
    pub features: &'static [&'static str],
    pub stack: &'static [&'static str],
    /// (image path, caption)
    pub shots: &'static [(&'static str, &'static str)],
}

pub const GAME: Game = Game {
    title: "Block Coders",
    pitch: "A blocky, Minecraft-style coding game, written in Rust, that teaches kids aged 8–12 to think like programmers. They snap code blocks together to guide a miner through 66 levels across seven worlds: sequences, loops, functions, if/else, while loops and algorithms, and variables.",
    repo: "https://github.com/rpforonda/block-coders",
    play_url: "play/index.html",
    features: &[
        "34 dynamic levels that reshuffle on every run, so memorized paths fail and real logic wins",
        "Anti-luck checks: winning programs are quietly re-tested against 80 variants of the level",
        "Step-through debugger that highlights the running block, and smart hints that read the kid's own program",
        "\"Show Rust code\" turns block programs into real, syntax-highlighted Rust",
        "Built for kids new to computers: a guided tour, readable text (WCAG AA contrast, the Atkinson Hyperlegible font) and layouts for phones held either way",
        "A tiny compiler and VM, and procedurally generated pixel art and sound",
    ],
    stack: &["Rust", "macroquad", "WebAssembly", "Custom VM"],
    shots: &[
        ("public/block-coders/if-else-level.webp", "A dynamic If/Else level in the Decision Desert, running"),
        ("public/block-coders/rust-code.webp", "Show Rust code: a function with while loops"),
        ("public/block-coders/functions.webp", "A function running in its own tab"),
        ("public/block-coders/variables.webp", "Counting Caverns: the count variable shown as a box while the program runs"),
        ("public/block-coders/tour.webp", "The guided tour on level 1"),
        ("public/block-coders/levels.webp", "Level select: seven worlds"),
    ],
};
