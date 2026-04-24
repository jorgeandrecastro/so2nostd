# so2nostd

[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL%202.0%2B-blue.svg)](https://www.gnu.org/licenses/gpl-2.0)
[![No Std](https://img.shields.io/badge/no_std-compatible-green.svg)](https://docs.rs/so2nostd)
[![Maintenance](https://img.shields.io/badge/Maintenance-Actively--developed-brightgreen.svg)](https://github.com/jorgeandrecastro/so2nostd)

## Contrôleur du Second Ordre `no_std` pour Systèmes Embarqués,testé sur une RP2040.

**so2nostd** est une crate Rust légère et performante `no_std` implémentant un système de contrôle du second ordre (SO2) en temps discret. Conçue pour les environnements embarqués comme les microcontrôleurs (par ex., RP2040), elle fournit une dynamique stable basée sur la physique utilisant l'intégration d'Euler.
# Mise à jour Version 0.2.1
#![forbid(unsafe_code)] pour la sécurité et opt-level = 3 pour la vitesse

Sous licence GPL-2.0-or-later pour garantir la protection de la communauté contre la privatisation. Optimisée pour une empreinte minimale et une fiabilité maximale.

## Table des Matières
- [Caractéristiques](#caractéristiques)
- [Installation](#installation)
- [Démarrage Rapide](#démarrage-rapide)
- [Référence API](#référence-api)
- [Performance et Optimisation](#performance-et-optimisation)
- [Tests](#tests)
- [Licence](#licence)

## 🚀 Caractéristiques
- ✅ **`no_std` Pur** : Zéro dépendance de la bibliothèque standard, parfait pour bare-metal/RTOS.
- ⚡ **Virgule Flottante Flexible** : `f64` (par défaut) ou feature `f32` pour appareils à ressources limitées.
- 🔧 **Optimisé en Taille** : Compatible avec `opt-level="z"`, LTO et `strip = true`.
- 🛡️ **Sécurité Numérique** : Gère `dt <= 0`, NaN/Inf et prévient la divergence.
- 📈 **SO2 Basé sur la Physique** : Modélise la fréquence naturelle (`ω_n`), le rapport d'amortissement (`ζ`) et le gain statique.
- 🎯 **Suivi de Consigne** : Convergence progressive vers la `consigne`.
- ⛑️ **Limites de Sécurité Optionnelles** : `max_velocity` et `max_acceleration` pour les contraintes de sécurité embarquée.

## 🛠️ Installation

Ajoutez à votre `Cargo.toml` :

````
[dependencies]
so2nostd = "0.2.2"


f32 pour embarqué (par ex., Cortex-M) :

so2nostd = {version="0.2.2", features = ["f32"] }

cargo add so2nostd
````

Compilez avec optimisations :

````rust
cargo build --release

````

# 🚀 Démarrage Rapide

````rust
use so2nostd::So2Controller;

fn main() {
    // ω_n=10 rad/s, ζ=0.7 (sous-amorti), initial=0.0, gain=1.0
    let mut controller = So2Controller::new(10.0, 0.7, 0.0, 1.0);

    controller.set_target(1.0); // Consigne désirée
    controller.set_max_velocity(0.5); // Limite de vitesse optionnelle
    controller.set_max_acceleration(10.0); // Limite d'accélération optionnelle

    let dt = 0.01; // Pas de temps de 10ms

    // Boucle de mise à jour du contrôleur
    for _ in 0..100 {
        let output = controller.update(0.0, dt);
        // output approche progressivement la consigne en suivant la dynamique SO2
    }
}
````


# 📚 Référence API


| Méthode                | Signature                                                                                | Description                                               |
| ---------------------- | ---------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| `new`                  | `So2Controller::new(w_n: Float, zeta: Float, initial_value: Float, gain: Float) -> Self` | Crée une nouvelle instance de contrôleur.                        |
| `update`               | `&mut self.update(input: Float, dt: Float) -> Float`                                     | Met à jour l'état du système vers la `consigne`. Sûr pour dt <= 0. |
| `set_target`           | `&mut self.set_target(target: Float)`                                                    | Met à jour la consigne interne.                            |
| `reset`                | `&mut self.reset(value: Float)`                                                          | Réinitialise les états (`y`, `y_prev`, `consigne`) à `value`.     |
| `set_max_velocity`     | `&mut self.set_max_velocity(max_v: Float)`                                               | Optionnel : limite la vélocité maximale.                            |
| `set_max_acceleration` | `&mut self.set_max_acceleration(max_a: Float)`                                           | Optionnel : limite l'accélération maximale.                        |
# Exemple d'utilisation : Contrôle de Trajectoire Asynchrone
Ce module permet d'intégrer un contrôle de mouvement fluide et déterministe (So2Controller) dans une tâche Embassy par exemple.

1. **Importation de la Crate**
````rust
use so2nostd::So2Controller;
````

2. **Implémentation de la Tâche de Contrôle**
La tâche gère la trajectoire de manière isolée, garantissant une mise à jour constante du signal PWM indépendamment du reste du système.

````rust
#[embassy_executor::task]
async fn servo_task(mut pwm: Pwm<'static>) {
    // Initialisation du contrôleur avec les paramètres de dynamique
    let mut controller = So2Controller::new(10.0, 0.7, 0.0, 1.0);
    controller.set_max_velocity(300.0);
    controller.set_max_acceleration(1000.0);

    loop {
        // Définition de la cible (ex: 180 degrés)
        let target = 180.0;
        controller.set_target(target);
        
        loop {
            // Mise à jour de la position (Pas de temps de 20ms)
            let pos = controller.update(target, 0.02);
            
            // Conversion et application au PWM
            let duty = ((pos / 180.0) * 65535.0) as u16;
            let _ = pwm.set_duty_cycle(duty);
            
            // Sortie de boucle si la cible est atteinte (seuil de précision)
            if (pos - target).abs() < 0.5 { break; }
            
            Timer::after_millis(20).await;
        }
        
        Timer::after_millis(1000).await;
    }
}
````

3. **Lancement dans le Main**

````rust
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // [...] Configuration du hardware
    
    // Lancement de la tâche de pilotage
    spawner.spawn(servo_task(pwm)).unwrap();
    
    loop {
        Timer::after_millis(1000).await;
    }
}
````


Type : Float = f64 (par défaut) ou f32 avec le flag de feature.

**Champs publics : w_n, zeta, setpoint, gain (inspectables/accordables).**

**⚡ Performance et Optimisation**
Taille binaire : Minimale, optimisée avec opt-level=3, LTO, strip = true.
Coût CPU : Temps constant O(1) par mise à jour.
Mémoire : Stack uniquement, zéro allocation.
Idéale pour les boucles de contrôle 100–10kHz sur les MCU.

**🧪 Tests**

Inclut des tests pour :

Stabilité de réponse indicielle et convergence.
Gestion du dt zéro/négatif.
Suivi de consigne et application des limites de sécurité.

Exécutez :
````
cargo test -- --nocapture
````

# ⚖️ Licence

GPL-2.0-or-later © 2026 Jorge Andre Castro.

Libre d'utilisation, de modification et de distribution. Tous les travaux dérivés doivent également être GPL-2.0-or-later.
