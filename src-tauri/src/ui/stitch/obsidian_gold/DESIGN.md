# Design System Specification

## 1. Overview & Creative North Star: "The Digital Atelier"

The creative north star for this design system is **The Digital Atelier**. This concept envisions the note-taking app not as a mere utility, but as a high-end, private sanctuary for thought. Inspired by editorial design and bespoke craftsmanship, the system rejects the "box-within-a-box" monotony of standard productivity tools. 

Instead of rigid grids, we utilize **intentional asymmetry** and **tonal depth**. The UI is treated as a series of floating, tactile surfaces. By leveraging generous whitespace (the "breathing room") and high-contrast typography scales, we transform a simple note-taking interface into a focused, professional environment where AI integration feels like a seamless whisper rather than a cluttered intrusion.

---

## 2. Colors: Tonal Depth & The "No-Line" Rule

Our palette is rooted in deep, obsidian grays and warm, earthy tones. The goal is to create a UI that recedes into the background, allowing the user's content to take center stage.

### The "No-Line" Rule
**Explicit Instruction:** Sectioning via 1px solid borders is strictly prohibited. Hierarchy and boundaries must be established solely through background color shifts or subtle tonal transitions. For example, a sidebar using `surface_container_low` should sit adjacent to a main editor using `surface`, separated only by the change in value.

### Surface Hierarchy & Nesting
Use the surface tiers to create "physical" layers. The app is a stack of fine materials:
- **Base Layer:** `surface` (#0f0e0d) — The infinite canvas.
- **Secondary Containers:** `surface_container` (#1b1918) — Primary navigation or list areas.
- **High-Focus Elements:** `surface_container_high` (#211f1d) — Active cards or selected states.
- **Glassmorphism:** For floating modals (like the "New Note" dialog), use `surface_container_highest` (#272523) at 80% opacity with a `24px` backdrop-blur.

### Signature Accents
- **Primary CTA:** Utilize `primary` (#e2c19f). For a more "editorial" feel, apply a subtle linear gradient from `primary` to `primary_container` (#594229) at a 135-degree angle. This provides a tactile "glow" that flat colors cannot replicate.

---

## 3. Typography: The Editorial Voice

We pair **Manrope** for expressive headlines and **Inter** for functional labels to create a sophisticated, high-contrast hierarchy.

| Level | Token | Font | Size | Weight | Character Spacing |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Display** | `display-lg` | Manrope | 3.5rem | 700 | -0.02em |
| **Headline** | `headline-md`| Manrope | 1.75rem | 600 | -0.01em |
| **Title** | `title-sm` | Manrope | 1.0rem | 600 | 0 |
| **Body** | `body-lg` | Manrope | 1.0rem | 400 | 0 |
| **Label** | `label-md` | Inter | 0.75rem | 500 | 0.05em (Caps) |

**Creative Usage:** Use `display-lg` for empty states or AI-generated summaries to create a "magazine" feel. Headlines should use tight tracking to feel authoritative, while labels should be slightly tracked out for legibility.

---

## 4. Elevation & Depth: Tonal Layering

This system abandons the traditional "drop shadow." We achieve lift through light and material properties.

- **The Layering Principle:** Depth is achieved by stacking. A `surface_container_lowest` card sitting on a `surface_container_low` section creates a natural "valley" of focus without a single pixel of shadow.
- **Ambient Shadows:** For floating elements (e.g., the FAB or Modals), use a "Tinted Shadow."
    - **Value:** `0px 20px 40px rgba(0, 0, 0, 0.4)`
    - **Softness:** Shadows must be ultra-diffused. The shadow color should technically be a darker version of the `background` to mimic natural light absorption.
- **The Ghost Border Fallback:** If a boundary is required for accessibility, use the `outline_variant` (#4a4745) at **15% opacity**. It should be felt, not seen.

---

## 5. Components: Bespoke Interaction

### Buttons
- **Primary:** `primary` background with `on_primary` text. Use `xl` (1.5rem) roundedness.
- **Tertiary:** No background. Use `primary` text. On hover, apply a `surface_bright` ghost-fill at 10% opacity.

### Note Cards
- **Construction:** Use `surface_container_low`. 
- **Constraint:** No dividers. Use `1.5rem` (xl) vertical spacing between cards.
- **Interaction:** On hover, shift the background to `surface_container_high`. Do not move the card; the shift should be purely tonal.

### Input Fields
- **Styling:** Use `surface_container_lowest` (#000000) for the field background to create a "well" effect.
- **Active State:** Instead of a thick border, use a `1px` "Ghost Border" at 40% opacity using the `primary` token.

### The AI Integrated FAB
- **Visuals:** A circular button using `primary` (#e2c19f). 
- **Elevation:** This is the only element allowed a significant Ambient Shadow to signify its power as the "Entry Point" for the AI assistant.

---

## 6. Do's and Don'ts

### Do
- **Do** use `2rem` to `4rem` of padding for main editor views. Space is a luxury; use it.
- **Do** use `secondary` (#a59c96) for "Coming Soon" or disabled meta-data to create a soft, muted look.
- **Do** align text-heavy components to a generous left margin to create an editorial "gutter."

### Don't
- **Don't** use 100% white (#ffffff) for text. Use `on_surface` (#eae4e0) to reduce eye strain in the dark theme.
- **Don't** use sharp corners. Every container must use at least the `DEFAULT` (0.5rem) roundedness to maintain the "soft minimalism" aesthetic.
- **Don't** use horizontal divider lines in lists. If you feel the need for a line, add more whitespace instead.