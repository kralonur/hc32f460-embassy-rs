#[doc = "Register `PVDCR0` reader"]
pub type R = crate::R<Pvdcr0Spec>;
#[doc = "Register `PVDCR0` writer"]
pub type W = crate::W<Pvdcr0Spec>;
#[doc = "Field `EXVCCINEN` reader - desc EXVCCINEN"]
pub type ExvccinenR = crate::BitReader;
#[doc = "Field `EXVCCINEN` writer - desc EXVCCINEN"]
pub type ExvccinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1EN` reader - desc PVD1EN"]
pub type Pvd1enR = crate::BitReader;
#[doc = "Field `PVD1EN` writer - desc PVD1EN"]
pub type Pvd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2EN` reader - desc PVD2EN"]
pub type Pvd2enR = crate::BitReader;
#[doc = "Field `PVD2EN` writer - desc PVD2EN"]
pub type Pvd2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EXVCCINEN"]
    #[inline(always)]
    pub fn exvccinen(&self) -> ExvccinenR {
        ExvccinenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - desc PVD1EN"]
    #[inline(always)]
    pub fn pvd1en(&self) -> Pvd1enR {
        Pvd1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PVD2EN"]
    #[inline(always)]
    pub fn pvd2en(&self) -> Pvd2enR {
        Pvd2enR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EXVCCINEN"]
    #[inline(always)]
    pub fn exvccinen(&mut self) -> ExvccinenW<'_, Pvdcr0Spec> {
        ExvccinenW::new(self, 0)
    }
    #[doc = "Bit 5 - desc PVD1EN"]
    #[inline(always)]
    pub fn pvd1en(&mut self) -> Pvd1enW<'_, Pvdcr0Spec> {
        Pvd1enW::new(self, 5)
    }
    #[doc = "Bit 6 - desc PVD2EN"]
    #[inline(always)]
    pub fn pvd2en(&mut self) -> Pvd2enW<'_, Pvdcr0Spec> {
        Pvd2enW::new(self, 6)
    }
}
#[doc = "desc PVDCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pvdcr0Spec;
impl crate::RegisterSpec for Pvdcr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pvdcr0::R`](R) reader structure"]
impl crate::Readable for Pvdcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`pvdcr0::W`](W) writer structure"]
impl crate::Writable for Pvdcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVDCR0 to value 0"]
impl crate::Resettable for Pvdcr0Spec {}
