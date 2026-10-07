#[doc = "Register `PDWKE0` reader"]
pub type R = crate::R<Pdwke0Spec>;
#[doc = "Register `PDWKE0` writer"]
pub type W = crate::W<Pdwke0Spec>;
#[doc = "Field `WKE00` reader - desc WKE00"]
pub type Wke00R = crate::BitReader;
#[doc = "Field `WKE00` writer - desc WKE00"]
pub type Wke00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE01` reader - desc WKE01"]
pub type Wke01R = crate::BitReader;
#[doc = "Field `WKE01` writer - desc WKE01"]
pub type Wke01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE02` reader - desc WKE02"]
pub type Wke02R = crate::BitReader;
#[doc = "Field `WKE02` writer - desc WKE02"]
pub type Wke02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE03` reader - desc WKE03"]
pub type Wke03R = crate::BitReader;
#[doc = "Field `WKE03` writer - desc WKE03"]
pub type Wke03W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE10` reader - desc WKE10"]
pub type Wke10R = crate::BitReader;
#[doc = "Field `WKE10` writer - desc WKE10"]
pub type Wke10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE11` reader - desc WKE11"]
pub type Wke11R = crate::BitReader;
#[doc = "Field `WKE11` writer - desc WKE11"]
pub type Wke11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE12` reader - desc WKE12"]
pub type Wke12R = crate::BitReader;
#[doc = "Field `WKE12` writer - desc WKE12"]
pub type Wke12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE13` reader - desc WKE13"]
pub type Wke13R = crate::BitReader;
#[doc = "Field `WKE13` writer - desc WKE13"]
pub type Wke13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc WKE00"]
    #[inline(always)]
    pub fn wke00(&self) -> Wke00R {
        Wke00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WKE01"]
    #[inline(always)]
    pub fn wke01(&self) -> Wke01R {
        Wke01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc WKE02"]
    #[inline(always)]
    pub fn wke02(&self) -> Wke02R {
        Wke02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WKE03"]
    #[inline(always)]
    pub fn wke03(&self) -> Wke03R {
        Wke03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc WKE10"]
    #[inline(always)]
    pub fn wke10(&self) -> Wke10R {
        Wke10R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WKE11"]
    #[inline(always)]
    pub fn wke11(&self) -> Wke11R {
        Wke11R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc WKE12"]
    #[inline(always)]
    pub fn wke12(&self) -> Wke12R {
        Wke12R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc WKE13"]
    #[inline(always)]
    pub fn wke13(&self) -> Wke13R {
        Wke13R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc WKE00"]
    #[inline(always)]
    pub fn wke00(&mut self) -> Wke00W<'_, Pdwke0Spec> {
        Wke00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc WKE01"]
    #[inline(always)]
    pub fn wke01(&mut self) -> Wke01W<'_, Pdwke0Spec> {
        Wke01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc WKE02"]
    #[inline(always)]
    pub fn wke02(&mut self) -> Wke02W<'_, Pdwke0Spec> {
        Wke02W::new(self, 2)
    }
    #[doc = "Bit 3 - desc WKE03"]
    #[inline(always)]
    pub fn wke03(&mut self) -> Wke03W<'_, Pdwke0Spec> {
        Wke03W::new(self, 3)
    }
    #[doc = "Bit 4 - desc WKE10"]
    #[inline(always)]
    pub fn wke10(&mut self) -> Wke10W<'_, Pdwke0Spec> {
        Wke10W::new(self, 4)
    }
    #[doc = "Bit 5 - desc WKE11"]
    #[inline(always)]
    pub fn wke11(&mut self) -> Wke11W<'_, Pdwke0Spec> {
        Wke11W::new(self, 5)
    }
    #[doc = "Bit 6 - desc WKE12"]
    #[inline(always)]
    pub fn wke12(&mut self) -> Wke12W<'_, Pdwke0Spec> {
        Wke12W::new(self, 6)
    }
    #[doc = "Bit 7 - desc WKE13"]
    #[inline(always)]
    pub fn wke13(&mut self) -> Wke13W<'_, Pdwke0Spec> {
        Wke13W::new(self, 7)
    }
}
#[doc = "desc PDWKE0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwke0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwke0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdwke0Spec;
impl crate::RegisterSpec for Pdwke0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdwke0::R`](R) reader structure"]
impl crate::Readable for Pdwke0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdwke0::W`](W) writer structure"]
impl crate::Writable for Pdwke0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDWKE0 to value 0"]
impl crate::Resettable for Pdwke0Spec {}
